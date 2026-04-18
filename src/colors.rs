use termcolor::{Color, ColorChoice, StandardStream, WriteColor};
use std::io::Write;

/// Determines if colors should be used based on:
/// 1. NO_COLOR env var (if set, disables colors)
/// 2. Platform (Windows CMD typically doesn't support colors)
fn should_use_colors() -> bool {
    // Check NO_COLOR environment variable
    if std::env::var("NO_COLOR").is_ok() {
        return false;
    }

    // On Windows, assume no colors unless explicitly enabled
    #[cfg(target_os = "windows")]
    {
        std::env::var("FORCE_COLOR").is_ok()
    }

    #[cfg(not(target_os = "windows"))]
    {
        true
    }
}

/// Print a colored error message
pub fn error(msg: &str) {
    if should_use_colors() {
        let mut stderr = StandardStream::stderr(ColorChoice::Always);
        let _ = stderr.set_color(termcolor::ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true));
        let _ = writeln!(stderr, "{}", msg);
        let _ = stderr.reset();
    } else {
        eprintln!("{}", msg);
    }
}

/// Print a colored warning message
pub fn warning(msg: &str) {
    if should_use_colors() {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        let _ = stdout.set_color(termcolor::ColorSpec::new().set_fg(Some(Color::Yellow)));
        let _ = writeln!(stdout, "{}", msg);
        let _ = stdout.reset();
    } else {
        println!("{}", msg);
    }
}

/// Print a colored success message
pub fn success(msg: &str) {
    if should_use_colors() {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        let _ = stdout.set_color(termcolor::ColorSpec::new().set_fg(Some(Color::Green)).set_bold(true));
        let _ = writeln!(stdout, "{}", msg);
        let _ = stdout.reset();
    } else {
        println!("{}", msg);
    }
}

/// Print a colored info message
pub fn info(msg: &str) {
    if should_use_colors() {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        let _ = stdout.set_color(termcolor::ColorSpec::new().set_fg(Some(Color::Cyan)));
        let _ = writeln!(stdout, "{}", msg);
        let _ = stdout.reset();
    } else {
        println!("{}", msg);
    }
}
