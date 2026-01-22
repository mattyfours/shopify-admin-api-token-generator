pub mod json_kit;
pub use json_kit::{
  read_json_file,
  write_json_file
};

pub mod logs;
pub use logs::{
  error_log,
  log_magenta,
  log_cyan,
  log_white,
  log_green
};

pub mod shopify_authentication;
pub use shopify_authentication::{
  get_code_from_oauth_url,
  exchange_code_for_token
};