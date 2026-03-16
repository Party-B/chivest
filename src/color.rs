use std::io::IsTerminal;

fn is_color() -> bool {
    std::io::stdout().is_terminal()
}

pub fn green(s: &str) -> String {
    if is_color() { format!("\x1b[32m{s}\x1b[0m") } else { s.to_string() }
}

pub fn red(s: &str) -> String {
    if is_color() { format!("\x1b[31m{s}\x1b[0m") } else { s.to_string() }
}

pub fn yellow(s: &str) -> String {
    if is_color() { format!("\x1b[33m{s}\x1b[0m") } else { s.to_string() }
}

pub fn dim(s: &str) -> String {
    if is_color() { format!("\x1b[2m{s}\x1b[0m") } else { s.to_string() }
}

pub fn bold(s: &str) -> String {
    if is_color() { format!("\x1b[1m{s}\x1b[0m") } else { s.to_string() }
}
