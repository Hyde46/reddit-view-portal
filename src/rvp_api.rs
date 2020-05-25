use crate::oauth::{authorize_user, curl_site, request_site, OAuthClient};
use crate::rvp_ui::{RVPUI};

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

    pub fn run(&mut self, loglevel: String) {
        let ui = RVPUI::new(loglevel);
        // Main Logic of RVP
        // Take command
        ui.print_welcome_message();
        let c = ui.expect_command();
        let command = self.validate_command(c).unwrap();
        self.execute_command(command);
    }

    fn validate_command(&self, c: String) -> Result<String, String> {
        match &c[..] {
            "l" | "login" => Ok(String::from("l")),
            "r" | "subreddit" => Ok(String::from("r")),
            _ => Err(String::from("Unknown command"))
        }
    }

    fn execute_command(&mut self, c: String) {
        match &c[..] {
            "l" => self.authorize_client(),
            _ => ()
        }
    }

    pub fn authorize_client(&mut self) {
        let o_client = authorize_user(self.client_config.auth_time);
        self.o_client = o_client;
    }
    pub fn get_subreddit_posts(&mut self, subreddit: &str, post_amount: usize) {

        use serde_json::{Result, Value};
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
