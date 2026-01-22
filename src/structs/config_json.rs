use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Store {
  pub admin_api_access_token: Option<String>,
  pub store_name: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ConfigJson {
  pub client_id: String,
  pub client_secret: String,
  pub redirect_url: String,
  pub scopes: String,
  pub stores: Vec<Store>
}