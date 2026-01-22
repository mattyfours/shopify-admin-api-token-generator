use std::fs;
use inquire::{
  error::InquireError,
  Select,
};

mod utils;
use utils::{
  read_json_file,
  write_json_file,
  error_log,
  log_magenta,
  log_green,
  get_code_from_oauth_url,
  exchange_code_for_token
};

mod structs;
use structs::{
  ConfigJson
};

const CONFIG_DIRECTORY: &str = "./app_configs";

#[tokio::main]
async fn main() {
  let app_config_options = fs::read_dir(CONFIG_DIRECTORY)
    .unwrap()
    .map(|entry| {
        let entry = entry.unwrap();
        entry.file_name().into_string().unwrap()
    })
    .filter(|file_name| file_name.ends_with(".json"))
    .collect::<Vec<String>>();

  let working_config_file_name: Result<String, InquireError> = Select::new("Select Config File", app_config_options)
    .prompt();

  let working_config_file_path = format!("{}/{}", CONFIG_DIRECTORY, working_config_file_name.unwrap());

  log_magenta(format!("Using config file: {:?}\n", working_config_file_path));

  let mut config_json: ConfigJson = read_json_file::<ConfigJson>(&working_config_file_path);

  let client_id = config_json.client_id.clone();
  let client_secret = config_json.client_secret.clone();
  let redirect_url = config_json.redirect_url.clone();

  if client_id.is_empty() || client_secret.is_empty() || redirect_url.is_empty() {
    error_log("Client ID, Client Secret, or Redirect URL is empty in the config file.");
  }

  let scopes = config_json.scopes
    .chars()
    .filter(|char| !char.is_whitespace())
    .collect::<String>();

  if scopes.is_empty() {
    error_log("Scopes are empty in the config file.");
  }

  let store_options = config_json.stores.iter()
    .map(|store| store.store_name.clone())
    .filter(|name| !name.is_empty())
    .collect::<Vec<String>>();

  let store_to_request: Result<String, InquireError> = Select::new("Select Store", store_options)
    .prompt();

  let store_my_shopify_url = format!(
    "https://{}.myshopify.com",
    store_to_request.as_ref().unwrap()
  );

  log_magenta(format!("Selected store: {:?}\n", store_my_shopify_url));

  let store_to_update = config_json.stores.iter_mut()
    .find(|store| store.store_name.to_string() == store_to_request.as_ref().unwrap().to_string())
    .unwrap();

  let existing_token = store_to_update.admin_api_access_token.clone().unwrap_or("".to_string());

  if !existing_token.is_empty() {
    let overwrite_token = Select::new(
      "Overwrite the existing token?",
      vec!["NO", "YES"]
    )
      .prompt()
      .unwrap();

    if overwrite_token == "NO" {
      log_magenta("Exiting without changes.".to_string());
      log_green(format!(
        "Existing Token: {:?}\n",
        existing_token
      ));
      return;
    }
  }

  let code = get_code_from_oauth_url(
    &client_id,
    &store_my_shopify_url,
    &redirect_url,
    &scopes
  );

  let token = exchange_code_for_token(
    &client_id,
    &store_my_shopify_url,
    &client_secret,
    &code
  ).await.unwrap();

  store_to_update.admin_api_access_token = Some(token);
  write_json_file(&working_config_file_path, &config_json);
}
