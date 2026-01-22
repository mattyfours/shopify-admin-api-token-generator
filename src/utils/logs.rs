pub fn error_log(message: &str) {
  panic!("\x1b[31m\nERROR: {}\n\x1b[0m", message);
}

pub fn log_magenta(message: String) {
  println!("\x1b[35m\n{}\n\x1b[0m", message);
}

pub fn log_cyan(message: String) {
  println!("\x1b[96m{}\x1b[0m", message);
}

pub fn log_white(message: String) {
  println!("\x1b[97m{}\x1b[0m", message);
}

pub fn log_green(message: String) {
  println!("\x1b[92m{}\x1b[0m", message);
}