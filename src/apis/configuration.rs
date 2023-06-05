/*
 * xethlyx's api server
 *
 * Public APIs for use within whatever you want. These are ratelimited, so try to keep requests to a minimum.  Future use of the API will require an API key.  ## API Keys API keys need to be passed alongside every request. With an API key, you are also expected to adhere to the following guidelines:   - Do not share your API key.   - Data can be retained for a maximum of 30 days (this is to adhere with GDPR guidelines).  API keys can also be saved to the browser for convenience using the client login/logout APIs.  You can obtain an API key by asking staff in the [Eclipsis discord](https://discord.gg/AkDsUtz). 
 *
 * The version of the OpenAPI document: 3.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */



#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest_middleware::ClientWithMiddleware,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub bearer_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
    // TODO: take an oauth2 token source, similar to the go one
}

pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}


impl Configuration {
    pub fn new() -> Configuration {
        Configuration::default()
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_path: "https://api.xethlyx.com".to_owned(),
            user_agent: Some("OpenAPI-Generator/3.0.0/rust".to_owned()),
            client: reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,

        }
    }
}