use std::{
    error::Error,
    io::{self, IsTerminal, Write},
    sync::{Arc, Mutex, MutexGuard, TryLockError},
    time::Duration,
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::memory::Value;

use super::{error::ExecutionError, execution::Execution, symbols};

type ResultValue = Result<Value, Box<dyn Error + Send + Sync>>;

static TERMINAL_GATE: Mutex<()> = Mutex::new(());
static TERMINAL_OWNER: Mutex<Option<Arc<()>>> = Mutex::new(None);
#[cfg(windows)]
const WINDOWS_PROCESSED_OUTPUT: u32 = 0x0001;
#[cfg(windows)]
const WINDOWS_VT_PROCESSING: u32 = 0x0004;

pub(crate) fn acquire() -> Result<MutexGuard<'static, ()>, Box<dyn Error + Send + Sync>> {
    match TERMINAL_GATE.try_lock() {
        Ok(guard) => Ok(guard),
        Err(TryLockError::WouldBlock) => Err("Terminal is already in use".into()),
        Err(TryLockError::Poisoned(error)) => Err(format!("Terminal lock: {error}").into()),
    }
}

trait Screen {
    fn is_terminal(&self) -> bool;
    fn raw_enabled(&self) -> io::Result<bool>;
    fn enable_raw(&mut self) -> io::Result<()>;
    fn disable_raw(&mut self) -> io::Result<()>;
    fn enter_alternate(&mut self) -> io::Result<()>;
    fn leave_alternate(&mut self) -> io::Result<()>;
    fn hide_cursor(&mut self) -> io::Result<()>;
    fn reset_colors(&mut self) -> io::Result<()>;
    fn show_cursor(&mut self) -> io::Result<()>;
}

#[derive(Default)]
struct CrosstermScreen {
    #[cfg(windows)]
    previous_input_mode: Option<u32>,
    #[cfg(windows)]
    previous_output_mode: Option<u32>,
    #[cfg(windows)]
    alternate_attempted: bool,
}

impl Screen for CrosstermScreen {
    fn is_terminal(&self) -> bool {
        io::stdin().is_terminal() && io::stdout().is_terminal()
    }
    fn raw_enabled(&self) -> io::Result<bool> {
        terminal::is_raw_mode_enabled()
    }
    fn enable_raw(&mut self) -> io::Result<()> {
        #[cfg(windows)]
        {
            let mode =
                crossterm_winapi::ConsoleMode::from(crossterm_winapi::Handle::current_in_handle()?);
            self.previous_input_mode = Some(mode.mode()?);
        }
        terminal::enable_raw_mode()
    }
    fn disable_raw(&mut self) -> io::Result<()> {
        #[cfg(windows)]
        {
            let Some(previous) = self.previous_input_mode else {
                return Ok(());
            };
            let mode =
                crossterm_winapi::ConsoleMode::from(crossterm_winapi::Handle::current_in_handle()?);
            mode.set_mode(previous)
        }
        #[cfg(not(windows))]
        {
            terminal::disable_raw_mode()
        }
    }
    fn enter_alternate(&mut self) -> io::Result<()> {
        #[cfg(windows)]
        {
            let mode = crossterm_winapi::ConsoleMode::from(
                crossterm_winapi::Handle::current_out_handle()?,
            );
            let previous = mode.mode()?;
            self.previous_output_mode = Some(previous);
            mode.set_mode(previous | WINDOWS_PROCESSED_OUTPUT | WINDOWS_VT_PROCESSING)?;
            self.alternate_attempted = true;
        }
        execute!(io::stdout(), EnterAlternateScreen)?;
        #[cfg(windows)]
        {
            let mode = crossterm_winapi::ConsoleMode::from(
                crossterm_winapi::Handle::current_out_handle()?,
            );
            mode.set_mode(mode.mode()? | WINDOWS_PROCESSED_OUTPUT | WINDOWS_VT_PROCESSING)?;
        }
        Ok(())
    }
    fn leave_alternate(&mut self) -> io::Result<()> {
        #[cfg(windows)]
        let result = if self.alternate_attempted {
            execute!(io::stdout(), LeaveAlternateScreen)
        } else {
            Ok(())
        };
        #[cfg(not(windows))]
        let result = execute!(io::stdout(), LeaveAlternateScreen);
        #[cfg(windows)]
        if let Some(previous) = self.previous_output_mode.take() {
            let restored = crossterm_winapi::Handle::current_out_handle()
                .and_then(|handle| crossterm_winapi::ConsoleMode::from(handle).set_mode(previous));
            return match (result, restored) {
                (Err(error), Err(restore)) => Err(io::Error::other(format!(
                    "leave alternate screen: {error}; restore output mode: {restore}"
                ))),
                (Err(error), Ok(())) => Err(error),
                (Ok(()), Err(error)) => Err(error),
                (Ok(()), Ok(())) => Ok(()),
            };
        }
        result
    }
    fn hide_cursor(&mut self) -> io::Result<()> {
        execute!(io::stdout(), Hide)
    }
    fn reset_colors(&mut self) -> io::Result<()> {
        execute!(io::stdout(), ResetColor)
    }
    fn show_cursor(&mut self) -> io::Result<()> {
        execute!(io::stdout(), Show)
    }
}

struct Session<S: Screen> {
    screen: S,
    _guard: MutexGuard<'static, ()>,
    raw_changed: bool,
    alternate_requested: bool,
    output_changed: bool,
    owner_set: bool,
    closed: bool,
}

impl<S: Screen> Session<S> {
    fn open(screen: S, owner: Arc<()>) -> Result<Self, Box<dyn Error + Send + Sync>> {
        if !screen.is_terminal() {
            return Err("Terminal requires a terminal on stdin and stdout".into());
        }
        let mut session = Self {
            screen,
            _guard: acquire()?,
            raw_changed: false,
            alternate_requested: false,
            output_changed: false,
            owner_set: false,
            closed: false,
        };
        let result = (|| -> Result<(), Box<dyn Error + Send + Sync>> {
            if !session.screen.raw_enabled()? {
                session.raw_changed = true;
                session.screen.enable_raw()?;
            }
            session.alternate_requested = true;
            session.screen.enter_alternate()?;
            session.output_changed = true;
            session.screen.hide_cursor()?;
            *TERMINAL_OWNER
                .lock()
                .map_err(|error| format!("Terminal owner lock: {error}"))? = Some(owner);
            session.owner_set = true;
            Ok(())
        })();
        if let Err(error) = result {
            return match session.close() {
                Ok(()) => Err(error),
                Err(cleanup) => Err(ExecutionError::at(
                    error,
                    format!("terminal cleanup failed: {cleanup}"),
                )),
            };
        }
        Ok(session)
    }

    fn close(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        if self.closed {
            return Ok(());
        }
        self.closed = true;
        let mut errors = Vec::new();
        if self.owner_set {
            match TERMINAL_OWNER.lock() {
                Ok(mut owner) => *owner = None,
                Err(error) => errors.push(format!("Terminal owner lock: {error}")),
            }
        }
        if self.output_changed {
            if let Err(error) = self.screen.reset_colors() {
                errors.push(format!("reset colors: {error}"));
            }
            if let Err(error) = self.screen.show_cursor() {
                errors.push(format!("show cursor: {error}"));
            }
        }
        if self.alternate_requested {
            if let Err(error) = self.screen.leave_alternate() {
                errors.push(format!("leave alternate screen: {error}"));
            }
        }
        if self.raw_changed {
            if let Err(error) = self.screen.disable_raw() {
                errors.push(format!("disable raw mode: {error}"));
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(format!("Terminal cleanup failed: {}", errors.join("; ")).into())
        }
    }
}

impl<S: Screen> Drop for Session<S> {
    fn drop(&mut self) {
        if let Err(error) = self.close() {
            eprintln!("{error}");
        }
    }
}

fn with_session<S: Screen>(
    screen: S,
    owner: Arc<()>,
    body: impl FnOnce() -> ResultValue,
) -> ResultValue {
    let mut session = Session::open(screen, owner)?;
    let result = body();
    match (result, session.close()) {
        (result, Ok(())) => result,
        (Ok(_), Err(cleanup)) => Err(cleanup),
        (Err(error), Err(cleanup)) => Err(ExecutionError::at(
            error,
            format!("terminal cleanup failed: {cleanup}"),
        )),
    }
}

fn require_owner(caller: &Execution) -> Result<(), Box<dyn Error + Send + Sync>> {
    let run_id = caller.terminal_run_id()?;
    let owner = TERMINAL_OWNER
        .lock()
        .map_err(|error| format!("Terminal owner lock: {error}"))?;
    if owner
        .as_ref()
        .is_some_and(|owner| Arc::ptr_eq(owner, &run_id))
    {
        Ok(())
    } else {
        Err("Terminal operation requires an active Terminal with: session".into())
    }
}

fn array(values: Vec<Value>) -> Value {
    Value::Array(Arc::new(Mutex::new(values)))
}

fn symbol(value: &'static str) -> Value {
    Value::Symbol(value)
}

fn encode_event(event: Event) -> ResultValue {
    match event {
        Event::Resize(columns, rows) => Ok(array(vec![
            symbol("#resize"),
            Value::Integer(columns.into()),
            Value::Integer(rows.into()),
        ])),
        Event::Key(key) => {
            let code = match key.code {
                KeyCode::Char(character) => Value::Character(character),
                KeyCode::F(number) => Value::Integer(number.into()),
                KeyCode::Backspace => symbol("#backspace"),
                KeyCode::Enter => symbol("#enter"),
                KeyCode::Left => symbol("#left"),
                KeyCode::Right => symbol("#right"),
                KeyCode::Up => symbol("#up"),
                KeyCode::Down => symbol("#down"),
                KeyCode::Home => symbol("#home"),
                KeyCode::End => symbol("#end"),
                KeyCode::PageUp => symbol("#pageUp"),
                KeyCode::PageDown => symbol("#pageDown"),
                KeyCode::Tab => symbol("#tab"),
                KeyCode::BackTab => symbol("#backTab"),
                KeyCode::Delete => symbol("#delete"),
                KeyCode::Insert => symbol("#insert"),
                KeyCode::Null => symbol("#null"),
                KeyCode::Esc => symbol("#escape"),
                KeyCode::CapsLock => symbol("#capsLock"),
                KeyCode::ScrollLock => symbol("#scrollLock"),
                KeyCode::NumLock => symbol("#numLock"),
                KeyCode::PrintScreen => symbol("#printScreen"),
                KeyCode::Pause => symbol("#pause"),
                KeyCode::Menu => symbol("#menu"),
                KeyCode::KeypadBegin => symbol("#keypadBegin"),
                other => return Err(format!("Unsupported terminal key: {other:?}").into()),
            };
            let mut modifiers = Vec::new();
            for (flag, name) in [
                (KeyModifiers::SHIFT, "#shift"),
                (KeyModifiers::CONTROL, "#control"),
                (KeyModifiers::ALT, "#alt"),
                (KeyModifiers::SUPER, "#super"),
                (KeyModifiers::HYPER, "#hyper"),
                (KeyModifiers::META, "#meta"),
            ] {
                if key.modifiers.contains(flag) {
                    modifiers.push(symbol(name));
                }
            }
            let kind = match key.kind {
                KeyEventKind::Press => "#press",
                KeyEventKind::Repeat => "#repeat",
                KeyEventKind::Release => "#release",
            };
            Ok(array(vec![
                symbol("#key"),
                code,
                array(modifiers),
                symbol(kind),
            ]))
        }
        other => Err(format!("Unsupported terminal event: {other:?}").into()),
    }
}

fn color(value: &Value) -> Result<Color, Box<dyn Error + Send + Sync>> {
    let Value::Symbol(name) = value else {
        return Err("Terminal color must be a symbol".into());
    };
    Ok(match symbols::text(name)? {
        "reset" => Color::Reset,
        "black" => Color::Black,
        "darkGrey" => Color::DarkGrey,
        "red" => Color::Red,
        "darkRed" => Color::DarkRed,
        "green" => Color::Green,
        "darkGreen" => Color::DarkGreen,
        "yellow" => Color::Yellow,
        "darkYellow" => Color::DarkYellow,
        "blue" => Color::Blue,
        "darkBlue" => Color::DarkBlue,
        "magenta" => Color::Magenta,
        "darkMagenta" => Color::DarkMagenta,
        "cyan" => Color::Cyan,
        "darkCyan" => Color::DarkCyan,
        "white" => Color::White,
        "grey" => Color::Grey,
        _ => return Err(format!("Unknown terminal color: {name}").into()),
    })
}

pub(crate) fn primitive(
    caller: &Execution,
    primitive: &str,
    receiver: &Value,
    args: &[Value],
) -> ResultValue {
    let Value::Class(class) = receiver else {
        return Err("Terminal primitive requires the Terminal class".into());
    };
    if class.name()? != "Terminal" {
        return Err("Terminal primitive requires the Terminal class".into());
    }
    if primitive == "VMpr_Terminal_with" {
        let [Value::Execution(block)] = args else {
            return Err("Terminal with: requires a block".into());
        };
        return with_session(
            CrosstermScreen::default(),
            caller.terminal_run_id()?,
            || block.invoke_block(caller, std::slice::from_ref(receiver)),
        );
    }
    require_owner(caller)?;
    let mut stdout = io::stdout();
    match (primitive, args) {
        ("VMpr_Terminal_size", []) => {
            let (columns, rows) = terminal::size()?;
            Ok(array(vec![
                Value::Integer(columns.into()),
                Value::Integer(rows.into()),
            ]))
        }
        ("VMpr_Terminal_clear", []) => {
            execute!(stdout, Clear(ClearType::All))?;
            Ok(receiver.clone())
        }
        ("VMpr_Terminal_move", [Value::Integer(column), Value::Integer(row)]) => {
            let column = u16::try_from(*column)
                .map_err(|_| "Terminal column must be between 0 and 65535")?;
            let row =
                u16::try_from(*row).map_err(|_| "Terminal row must be between 0 and 65535")?;
            execute!(stdout, MoveTo(column, row))?;
            Ok(receiver.clone())
        }
        ("VMpr_Terminal_write", [value]) => {
            match value {
                Value::String(text) => {
                    let text = text.to_rust_string()?;
                    execute!(stdout, Print(text))?
                }
                Value::Character(character) => execute!(stdout, Print(character))?,
                _ => return Err("Terminal write: requires a string or character".into()),
            }
            Ok(receiver.clone())
        }
        ("VMpr_Terminal_flush", []) => {
            stdout.flush()?;
            Ok(receiver.clone())
        }
        ("VMpr_Terminal_foreground", [value]) => {
            let color = color(value)?;
            execute!(stdout, SetForegroundColor(color))?;
            Ok(receiver.clone())
        }
        ("VMpr_Terminal_background", [value]) => {
            let color = color(value)?;
            execute!(stdout, SetBackgroundColor(color))?;
            Ok(receiver.clone())
        }
        ("VMpr_Terminal_resetColors", []) => {
            execute!(stdout, ResetColor)?;
            Ok(receiver.clone())
        }
        ("VMpr_Terminal_hideCursor", []) => {
            execute!(stdout, Hide)?;
            Ok(receiver.clone())
        }
        ("VMpr_Terminal_showCursor", []) => {
            execute!(stdout, Show)?;
            Ok(receiver.clone())
        }
        ("VMpr_Terminal_nextEvent", []) => encode_event(event::read()?),
        ("VMpr_Terminal_pollEvent", [Value::Integer(milliseconds)]) => {
            let milliseconds = u64::try_from(*milliseconds)
                .map_err(|_| "Terminal timeout must be non-negative")?;
            if event::poll(Duration::from_millis(milliseconds))? {
                encode_event(event::read()?)
            } else {
                Ok(Value::Nil)
            }
        }
        _ => Err(format!("Invalid terminal primitive arguments: {primitive}").into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyEvent;

    struct FakeScreen {
        calls: Arc<Mutex<Vec<&'static str>>>,
        raw: bool,
        fail: Option<&'static str>,
    }

    impl FakeScreen {
        fn step(&mut self, name: &'static str) -> io::Result<()> {
            self.calls.lock().unwrap().push(name);
            if self.fail == Some(name) {
                Err(io::Error::other(name))
            } else {
                Ok(())
            }
        }
    }

    impl Screen for FakeScreen {
        fn is_terminal(&self) -> bool {
            true
        }
        fn raw_enabled(&self) -> io::Result<bool> {
            Ok(self.raw)
        }
        fn enable_raw(&mut self) -> io::Result<()> {
            self.step("enable")
        }
        fn disable_raw(&mut self) -> io::Result<()> {
            self.step("disable")
        }
        fn enter_alternate(&mut self) -> io::Result<()> {
            self.step("enter")
        }
        fn leave_alternate(&mut self) -> io::Result<()> {
            self.step("leave")
        }
        fn hide_cursor(&mut self) -> io::Result<()> {
            self.step("hide")
        }
        fn reset_colors(&mut self) -> io::Result<()> {
            self.step("reset")
        }
        fn show_cursor(&mut self) -> io::Result<()> {
            self.step("show")
        }
    }

    #[test]
    fn session_restores_on_success_failure_and_partial_initialization() {
        let calls = Arc::new(Mutex::new(Vec::new()));
        let screen = |raw, fail| FakeScreen {
            calls: calls.clone(),
            raw,
            fail,
        };
        let owner = Arc::new(());
        let mut session = Session::open(screen(false, None), owner.clone()).unwrap();
        assert!(acquire().is_err());
        assert!(Arc::ptr_eq(
            TERMINAL_OWNER.lock().unwrap().as_ref().unwrap(),
            &owner
        ));
        session.close().unwrap();
        drop(session);
        assert_eq!(
            *calls.lock().unwrap(),
            [
                "enable", "enter", "hide", "reset", "show", "leave", "disable"
            ]
        );
        assert!(TERMINAL_OWNER.lock().unwrap().is_none());
        calls.lock().unwrap().clear();

        let session = Session::open(screen(true, None), owner.clone()).unwrap();
        drop(session);
        assert_eq!(
            *calls.lock().unwrap(),
            ["enter", "hide", "reset", "show", "leave"]
        );
        calls.lock().unwrap().clear();

        let error = match Session::open(screen(false, Some("hide")), owner.clone()) {
            Ok(_) => panic!("expected initialization failure"),
            Err(error) => error,
        };
        assert!(error.to_string().contains("hide"));
        assert_eq!(
            *calls.lock().unwrap(),
            [
                "enable", "enter", "hide", "reset", "show", "leave", "disable"
            ]
        );
        calls.lock().unwrap().clear();

        let mut session = Session::open(screen(false, Some("leave")), owner).unwrap();
        let error = session.close().unwrap_err().to_string();
        assert!(error.contains("leave alternate screen"));
        assert_eq!(
            *calls.lock().unwrap(),
            [
                "enable", "enter", "hide", "reset", "show", "leave", "disable"
            ]
        );
        drop(session);
        assert!(acquire().is_ok());
        calls.lock().unwrap().clear();

        let error = with_session(screen(false, None), Arc::new(()), || {
            Err("block failed".into())
        })
        .unwrap_err();
        assert_eq!(error.to_string(), "block failed");
        assert_eq!(
            *calls.lock().unwrap(),
            [
                "enable", "enter", "hide", "reset", "show", "leave", "disable"
            ]
        );
        calls.lock().unwrap().clear();
        let error = with_session(screen(false, Some("leave")), Arc::new(()), || {
            Err("block failed".into())
        })
        .unwrap_err()
        .to_string();
        assert!(error.contains("block failed"), "{error}");
        assert!(error.contains("leave alternate screen"), "{error}");
        assert_eq!(
            *calls.lock().unwrap(),
            [
                "enable", "enter", "hide", "reset", "show", "leave", "disable"
            ]
        );
    }

    fn elements(value: Value) -> Vec<Value> {
        let Value::Array(values) = value else {
            panic!("expected array");
        };
        let result = values.lock().unwrap().clone();
        result
    }

    #[test]
    fn events_preserve_key_kind_modifiers_and_resize_dimensions() {
        let event = KeyEvent::new_with_kind(
            KeyCode::Char('x'),
            KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            KeyEventKind::Repeat,
        );
        let values = elements(encode_event(Event::Key(event)).unwrap());
        assert!(matches!(
            values.as_slice(),
            [
                Value::Symbol("#key"),
                Value::Character('x'),
                Value::Array(_),
                Value::Symbol("#repeat")
            ]
        ));
        assert!(matches!(
            elements(values[2].clone()).as_slice(),
            [Value::Symbol("#shift"), Value::Symbol("#control")]
        ));
        assert!(matches!(
            elements(encode_event(Event::Resize(120, 40)).unwrap()).as_slice(),
            [
                Value::Symbol("#resize"),
                Value::Integer(120),
                Value::Integer(40)
            ]
        ));
        let event = KeyEvent::new(KeyCode::F(5), KeyModifiers::NONE);
        assert!(matches!(
            elements(encode_event(Event::Key(event)).unwrap()).as_slice(),
            [
                Value::Symbol("#key"),
                Value::Integer(5),
                Value::Array(_),
                Value::Symbol("#press")
            ]
        ));
        assert!(encode_event(Event::FocusGained).is_err());
    }
}
