use std::collections::HashMap;
use dotenv_codegen::dotenv;
use gloo::console::log;
use url::Url;



pub fn get_google_url(new_random:String) -> String {
    let client_id = std::env!("GOOGLE_OAUTH_CLIENT_ID");
    let redirect_uri = dotenv!("GOOGLE_REDIRECT_URL");
    //let redirect_uri = std::env!("");
    log!("redirect_uri :{}",redirect_uri);

    let root_url = "https://accounts.google.com/o/oauth2/v2/auth";
    let mut options = HashMap::new();
    options.insert("client_id", client_id);
    options.insert("redirect_uri", redirect_uri);
    options.insert("response_type", "code");
    options.insert(
        "scope",
        "https://www.googleapis.com/auth/userinfo.profile https://www.googleapis.com/auth/userinfo.email",
    );
    options.insert("access_type", "offline");
    options.insert("state", new_random.as_str());
    options.insert("prompt", "consent");



    let url = Url::parse_with_params(root_url, &options).unwrap();
    let qs = url.query().unwrap();
   
    format!("{}?{}", root_url, qs)
}

pub fn get_paypal_url(cuisine_id:String,token:String) -> String {
    let root_url = std::env!("PAYPAL_PAY_FOR_ADDR");

    let mut options = HashMap::new();
    options.insert("cuisine_id", cuisine_id);
    options.insert("token",token);
    let url = Url::parse_with_params(root_url, &options).unwrap();
    let qs = url.query().unwrap();

    format!("{}?{}", root_url, qs)
}