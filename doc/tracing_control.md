# Tracing Control Guide

## Method 1: Environment Variable Control (RUST_LOG)

With the updated `init_tracing()` function, you can control tracing using the `RUST_LOG` environment variable:

### Basic Usage

```bash
# Enable all tracing for the entire application
set RUST_LOG=trace
cargo test

# Enable specific modules at different levels
set RUST_LOG=st::parser=debug,st::compiler=info,st::vm=warn
cargo test

# Complex filtering
set RUST_LOG=st=trace,st::parser::topdown=debug,st::compiler=off
cargo test
```

### Filtering Syntax

- `st=trace` - Enable TRACE level for entire `st` crate
- `st::parser=debug` - DEBUG level for parser module only
- `st::parser::topdown=info` - INFO level for specific submodule
- `off` - Disable tracing for a module
- Multiple modules: `module1=level1,module2=level2`

### Log Levels (most to least verbose)
1. `trace` - Most detailed
2. `debug` - Debug information  
3. `info` - General information
4. `warn` - Warnings
5. `error` - Errors only
6. `off` - No logging

## Method 2: Programmatic Filtering

Create different tracing profiles in code:

```rust
pub fn init_tracing_profile(profile: &str) {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
    
    let filter = match profile {
        "parser-only" => "st::parser=trace,st=warn".into(),
        "compiler-debug" => "st::compiler=debug,st::parser=info,st=warn".into(),
        "vm-focus" => "st::vm=trace,st=info".into(),
        "quiet" => "st=warn".into(),
        "verbose" => "st=trace".into(),
        _ => "st=info".into(), // default
    };
    
    tracing_subscriber::registry()
        .with(EnvFilter::new(filter))
        .with(tracing_subscriber::fmt::layer())
        .init();
}
```

## Method 3: Conditional Compilation

Use features in Cargo.toml for compile-time control:

### Add to Cargo.toml:
```toml
[features]
default = []
trace-parser = []
trace-compiler = []
trace-vm = []
trace-all = ["trace-parser", "trace-compiler", "trace-vm"]
```

### In your code:
```rust
#[cfg_attr(feature = "trace-parser", tracing::instrument)]
pub fn parse_eval(src: &str) -> Result<SmalltalkNode, ParseError> {
    // function body
}

#[cfg(feature = "trace-compiler")]
#[tracing::instrument]
pub fn compile_method(node: &SmalltalkNode) -> CompiledMethod {
    // function body
}

// Conditional tracing calls
#[cfg(feature = "trace-parser")]
tracing::debug!("Parsing started for: {}", src);
```

### Build with specific features:
```bash
# Enable parser tracing only
cargo test --features trace-parser

# Enable all tracing
cargo test --features trace-all
```

## Method 4: Dynamic Module Control

Create a runtime configuration system:

```rust
use std::collections::HashMap;
use std::sync::RwLock;

pub struct TracingConfig {
    module_levels: RwLock<HashMap<String, tracing::Level>>,
}

impl TracingConfig {
    pub fn new() -> Self {
        let mut levels = HashMap::new();
        levels.insert("parser".to_string(), tracing::Level::INFO);
        levels.insert("compiler".to_string(), tracing::Level::WARN);
        levels.insert("vm".to_string(), tracing::Level::ERROR);
        
        Self {
            module_levels: RwLock::new(levels),
        }
    }
    
    pub fn set_level(&self, module: &str, level: tracing::Level) {
        self.module_levels.write().unwrap().insert(module.to_string(), level);
    }
    
    pub fn should_trace(&self, module: &str, level: &tracing::Level) -> bool {
        let levels = self.module_levels.read().unwrap();
        levels.get(module).map_or(false, |mod_level| level <= mod_level)
    }
}

// Usage in your code:
lazy_static::lazy_static! {
    static ref TRACING_CONFIG: TracingConfig = TracingConfig::new();
}

macro_rules! trace_if_enabled {
    ($module:expr, $level:expr, $($arg:tt)*) => {
        if TRACING_CONFIG.should_trace($module, &tracing::Level::TRACE) {
            tracing::trace!($($arg)*);
        }
    };
}
```

## Method 5: Test-Specific Tracing

For different tracing in tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    fn init_test_tracing(filter: &str) {
        use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
        
        let _ = tracing_subscriber::registry()
            .with(EnvFilter::new(filter))
            .with(tracing_subscriber::fmt::layer().with_test_writer())
            .try_init();
    }
    
    #[test]
    fn test_parser_only() {
        init_test_tracing("st::parser=trace");
        // your test code
    }
    
    #[test] 
    fn test_compiler_debug() {
        init_test_tracing("st::compiler=debug,st::parser=info");
        // your test code
    }
}
```

## Quick Examples

### PowerShell (Windows):
```powershell
# Parser debugging
$env:RUST_LOG="st::parser=trace"; cargo test test_grammar

# Compiler focus
$env:RUST_LOG="st::compiler=debug,st::parser=warn"; cargo run

# Everything verbose
$env:RUST_LOG="trace"; cargo test

# Quiet mode
$env:RUST_LOG="error"; cargo test
```

### Bash/Linux:
```bash
# Parser debugging  
RUST_LOG=st::parser=trace cargo test test_grammar

# Multiple modules
RUST_LOG="st::parser=debug,st::vm=info" cargo test
```

## Best Practices

1. **Default to INFO level** for production
2. **Use module-specific filtering** to reduce noise
3. **Environment variables for development** - easy to change
4. **Features for compile-time control** - zero runtime cost
5. **Test-specific tracing** for debugging individual tests
6. **Document your module names** so team members know what to filter

This gives you complete control over which parts of your Smalltalk implementation produce tracing output!