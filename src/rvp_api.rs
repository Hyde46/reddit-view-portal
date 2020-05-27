use crate::oauth::{authorize_user, curl_site, request_site, OAuthClient};
use crate::reddit_structs::RedditPost;
use crate::rvp_ui::*;
use serde_json::Value as SerdeValue;

pub struct RVPClient {
    o_client: OAuthClient,
    client_config: RvpClientConfig,
    is_exiting: bool,
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
            is_exiting: false,
        }
    }

    pub fn run(&mut self) {
        let mut hist = RedditHistory::new();
        // Main Logic of RVP
        // Take command
        print_welcome_message();
        while !self.is_exiting {
            display_status(&hist);
            let c = expect_command();
            let command = self.validate_command(c).unwrap();
            self.execute_command(command, &mut hist);
        }
        display_message("Goodbye!");
        // Cleanup here if necessary
    }

    fn validate_command(&self, c: String) -> Result<String, String> {
        match &c[..] {
            "l" | "login" => Ok(String::from("l")),
            "r" | "subreddit" => Ok(String::from("r")),
            "v" | "posts" => Ok(String::from("v")),
            "x" | "exit" => Ok(String::from("x")),
            _ => Err(String::from("Unknown command")),
        }
    }

    fn execute_command(&mut self, c: String, history: &mut RedditHistory) {
        match &c[..] {
            "l" => self.authorize_client(),
            "r" => self.switch_page(history),
            "v" => self.show_posts(history),
            "x" => self.exit_client(),
            _ => (),
        }
    }

    pub fn authorize_client(&mut self) {
        let o_client = authorize_user(self.client_config.auth_time);
        self.o_client = o_client;
    }

    fn switch_page(&mut self, history: &mut RedditHistory) {
        display_message("Which page to visit? /<page_type>/<page>");
        display_message("To visit a subreddit try: /r/rust");
        let target_page = expect_input();
        let split_target_page: Vec<&str> = target_page.split("/").collect();
        if split_target_page.len() <= 2 {
            display_log_message(
                &format!("Invalid visit-page command: {}", target_page),
                String::from("ERROR"),
            );
            display_message("Invalid page to visit.");
            return;
        }
        history.set_target_page(split_target_page[1], split_target_page[2]);
        self.show_posts(history);
    }

    fn exit_client(&mut self) {
        display_log_message("Exiting Client", "TRACE".to_string());
        self.is_exiting = true;
    }

    fn show_posts(&mut self, history: &RedditHistory) {
        self.get_subreddit_posts(&history.current_page, history.page_limit);
    }

    pub fn get_subreddit_posts(&mut self, subreddit: &str, post_amount: usize) {
        let string_response = curl_site(subreddit, post_amount);
        let posts: SerdeValue = serde_json::from_str(&string_response).unwrap();
        let mut posts_decon: Vec<RedditPost> = Vec::new();
        for n in 0..post_amount {
            posts_decon.push(RedditPost {
                id: posts["data"]["children"][n]["data"]["id"]
                    .clone()
                    .to_string(),
                subreddit: posts["data"]["children"][n]["data"]["subreddit"]
                    .clone()
                    .to_string(),
                title: posts["data"]["children"][n]["data"]["title"]
                    .clone()
                    .to_string(),
                ups: 0,
                gilded: 0,
                link_flair_text: posts["data"]["children"][n]["data"]["link_flair_text"]
                    .clone()
                    .to_string(),
                author: posts["data"]["children"][n]["data"]["author"]
                    .clone()
                    .to_string(),
                permalink: posts["data"]["children"][n]["data"]["permalink"]
                    .clone()
                    .to_string(),
                url: posts["data"]["children"][n]["data"]["url"]
                    .clone()
                    .to_string(),
            });
            println!("{:?}", posts_decon[n]);
        }
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
