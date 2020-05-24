use crate::oauth::{authorize_user, curl_site, request_site, OAuthClient};
use crate::rvp_ui::{RVPUI};
use serde_json::{Result, Value};

pub struct RVPClient {
    o_client: OAuthClient,
    client_config: RvpClientConfig,
}

struct RvpClientConfig {
    auth_time: usize,
}

impl RVPClient {
    pub fn new() -> RVPClient {
        let config = RvpClientConfig { auth_time: 60 };
        let o_client = OAuthClient::new();
        RVPClient {
            o_client,
            client_config: config,
        }
    }

    pub fn run(self, loglevel: String) {
        let ui = RVPUI::new(loglevel);
        // Main Logic of RVP
        // Take command
        ui.print_welcome_message();
        ui.expect_command();
    }

    pub fn authorize_client(auth_time: usize) -> OAuthClient {
        authorize_user(auth_time)
    }
    pub fn get_subreddit_posts(&mut self, subreddit: &str, post_amount: usize) {
        let string_response = curl_site(subreddit, post_amount);
        //println!("{}",string_response);
        let post: Value = serde_json::from_str(&string_response).unwrap();
        //let subreddit_children = subreddit_data["children"].as_object().unwrap();
        println!("{:?}", post["data"]["children"][0]["data"]["title"]);
    }
    pub fn get_top_subreddit(&mut self, amount: usize, subreddit: &str) {
        curl_site(subreddit, amount);
    }
    pub fn get_profile_info(&mut self) {
        request_site(
            self.o_client.get_access_token(),
            "https://oauth.reddit.com/api/v1/me".to_string(),
        );
    }
}
