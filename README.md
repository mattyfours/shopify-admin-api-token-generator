## Getting started

- [Install Rust](https://rust-lang.org/tools/install/)
- Clone project
- Open root of the project
- Run `cargo build`

## Configurations

- Create a new app in your develop dashboard
- Set the required permissions
- Create a Custom Install Link and install the app on a shop
- Open the project in a code editior
- Create a new json file in the `app_configs` directory
  - Example: `./app_configs/my-test-app`
  - This directly will NOT be uploaded to GitHub.

The JSON file should look like the following:

```
{
  "client_id": "abc123987",
  "client_secret": "abc123987",
  "redirect_url": "https://example.com/auth/callback",
  "scopes": "read_customers,read_products",
  "stores": [
    {
      "store_name": "my-test-store-1"
    },
    {
      "store_name": "my-test-store-2"
    }
  ]
}
```

Client ID and Client Secret can be found in Settings in the Dev Dashboard
<img width="1100" height="311" alt="image" src="https://github.com/user-attachments/assets/66d0ff35-dc1b-4a1a-8495-d045696ad30d" />

Scopes and Redirect URL can be found in the Active Version in the Dev Dashboard
<img width="818" height="308" alt="image" src="https://github.com/user-attachments/assets/f0f02d16-cbe8-4f56-9f3d-68b7f9e809f1" />

Store Names can be found by visiting the target store admin

- Example: `https://admin.shopify.com/store/my-test-store-1` = `my-test-store-1`
- If the app is installed on multiple stores, more than one store can be added to the stores array

# Generating Admin API Token

- From the project root, run: `cargo run`
- Follow the CLI prompts
- If successful the CLI will generate the access token
- Tokens will also be added to the JSON file in `app_configs`
