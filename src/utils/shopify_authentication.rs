use urlencoding::encode;
use reqwest::Client;
use inquire::{Text};

use super::{
  log_magenta,
  log_white,
  log_cyan,
  log_green,
  error_log
};

pub fn get_code_from_oauth_url(
  client_id: &str,
  store_my_shopify_url: &str,
  redirect_url: &str,
  scopes: &str
) -> String {
  let oauth_url = format!(
    "{}/admin/oauth/authorize?client_id={}&scope={}&redirect_uri={}",
    store_my_shopify_url,
    client_id,
    encode(&scopes),
    encode(&redirect_url)
  );

  log_white("\nOpen OAuth URL In Browser:".to_string());
  log_cyan(format!("{}\n", oauth_url));

  let inputed_code = Text::new("Insert 'code' url parameter from redirected url:")
    .prompt()
    .unwrap();

  log_magenta(format!("Code: {:?}", inputed_code));

  return inputed_code;
}

pub async fn exchange_code_for_token(
  client_id: &str,
  store_my_shopify_url: &str,
  client_secret: &str,
  code: &str
) -> Result<String, Box<dyn std::error::Error>> {
  let request_url = format!(
    "{}/admin/oauth/access_token",
    store_my_shopify_url
  );

  let res = Client::new()
    .post(&request_url)
    .json(&serde_json::json!({
      "client_id": client_id,
      "client_secret": client_secret,
      "code": code
    }))
    .send()
    .await?;

    if !res.status().is_success() {
      let error_message = format!(
        "HTTP request failed with status: {}", res.status()
      );
      error_log(error_message.as_str());
    }

    let res_json: serde_json::Value = res.json()
      .await?;

    let access_token = res_json["access_token"]
      .as_str()
      .unwrap();

    log_green(format!("Admin API Token: {:?}\n", access_token));

    Ok(access_token.to_string())
}