use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::{
    io::{self, Write},
    sync::Mutex,
};


use crate::vm::execution::Execution;

use super::Debugger;

pub struct TerminalDebugger(Mutex<TerminalDebuggerData>);

impl TerminalDebugger {
    pub fn new() -> Self {
        TerminalDebugger(Mutex::new(TerminalDebuggerData {
            step_mode: true,
            show_registers: true,
            context_lines: 12,
        }))
    }
}

impl Debugger for TerminalDebugger {
    fn before_execute(&self, execution: Execution) {
        let mut data = self.0.try_lock().unwrap();
        if let Err(e) = data.display_execution_state(execution) {
            eprintln!("Terminal debugger error: {}", e);
        }
    }

    fn after_execute(&self, _execution: Execution) {
        let _data = self.0.try_lock().unwrap();
        // Implement post-execution debugging UI here
    }
}

#[derive(Default)]
struct TerminalDebuggerData {
    step_mode: bool,
    show_registers: bool,
    context_lines: usize,
}

impl TerminalDebuggerData {
    fn display_execution_state(
        &mut self,
        execution: Execution,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = io::stdout();

        // Clear screen and move to top
        execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        // Display header
        queue!(
            stdout,
            SetForegroundColor(Color::Cyan),
            Print("\n=== Smalltalk VM Debugger ===\n"),
            ResetColor
        )?;

        // Display current instruction pointer and method info
        queue!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print(format!(
                "Method: {} instructions, IP: {} Step-Mode:{}\n",
                execution.instructions().len(),
                execution.ip(),
                self.step_mode
            )),
            Print(format!("Registers: {} allocated\n", execution.reg_count())),
            ResetColor
        )?;

        // Display current and surrounding instructions
        self.display_instructions(execution.clone(), &mut stdout)?;

        // Display registers if enabled
        if self.show_registers {
            self.display_registers(execution.clone(), &mut stdout)?;
        }

        // Display controls
        queue!(
            stdout,
            Print("\n"),
            SetForegroundColor(Color::Green),
            Print("Controls: [n]ext, [s]tep, [r]egisters toggle, [c]ontinue, [q]uit\n"),
            ResetColor
        )?;

        stdout.flush()?;

        // Handle user input in step mode
        if self.step_mode {
            self.handle_input()?;
        }

        Ok(())
    }

    fn display_instructions(
        &self,
        execution: Execution,
        stdout: &mut io::Stdout,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let instructions = execution.instructions();
        let current_ip = execution.ip();

        queue!(stdout, Print("\nInstructions:\n"))?;

        // Calculate range of instructions to show
        let context = self.context_lines / 2;
        let start = current_ip.saturating_sub(context);
        let end = (start + self.context_lines).min(instructions.len());

        for (i, instruction) in instructions.iter().enumerate().take(end).skip(start) {
            if i == current_ip {
                // Highlight current instruction
                queue!(
                    stdout,
                    SetBackgroundColor(Color::DarkYellow),
                    SetForegroundColor(Color::Black),
                    Print(format!(">>> {:3}: {}\n", i, instruction)),
                    ResetColor
                )?;
            } else {
                queue!(stdout, Print(format!("    {:3}: {}\n", i, instruction)))?;
            }
        }

        Ok(())
    }

    fn display_registers(
        &self,
        execution: Execution,
        stdout: &mut io::Stdout,
    ) -> Result<(), Box<dyn std::error::Error>> {
        queue!(
            stdout,
            Print("\nRegisters:\n"),
            SetForegroundColor(Color::Magenta)
        )?;

        {
            let registers = execution.registers();
            for (i, value) in registers.iter().enumerate() {
                queue!(stdout, Print(format!("  r{}: {}\n", i, value)))?;
            }
        }
        queue!(stdout, ResetColor)?;
        Ok(())
    }

    fn handle_input(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            if let Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Release,
                ..
            }) = event::read()?
            {
                match code {
                    KeyCode::Char('n') => {
                        // Next instruction (stay in step mode)
                        break;
                    }
                    KeyCode::Char('s') => {
                        // Step mode toggle
                        self.step_mode = !self.step_mode;
                        println!("Step mode: {}", if self.step_mode { "ON" } else { "OFF" });
                        break;
                    }
                    KeyCode::Char('r') => {
                        // Toggle register display
                        self.show_registers = !self.show_registers;
                        println!(
                            "Register display: {}",
                            if self.show_registers { "ON" } else { "OFF" }
                        );
                        // Don't break, continue waiting for input
                    }
                    KeyCode::Char('c') => {
                        // Continue (disable step mode)
                        self.step_mode = false;
                        break;
                    }
                    KeyCode::Char('q') => {
                        // Quit
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}
