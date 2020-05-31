use crate::oauth::{authorize_user, curl_site, request_site, OAuthClient};
use crate::reddit_structs::*;
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

pub struct Command {
    pub base_command: String,
    pub parameter: String,
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
        // Take command
        print_welcome_message();
        while !self.is_exiting {
            display_status(&hist);
            let mut c: Command = expect_command();
            c = self.shorten_command(&c);
            if !self.validate_command(&c) {
                display_message("Not a valid command.");
                continue;
            }
            self.execute_command(c, &mut hist);
        }
        display_message("Goodbye!");
        // Cleanup here if necessary
    }

    fn shorten_command(&self, c: &Command) -> Command {
        Command {
            base_command: match &c.base_command[..] {
                "l" | "login" => String::from("l"),
                "r" | "subreddit" => String::from("r"),
                "v" | "posts" => String::from("v"),
                "x" | "exit" => String::from("x"),
                "u" | "user" => String::from("u"),
                "q" | "logout" => String::from("q"),
                "c" | "create" => String::from("c"),
                _ => String::from("Unknown command"),
            },
            parameter: c.parameter.clone(),
        }
    }

    fn validate_command(&self, c: &Command) -> bool {
        match &c.base_command[..] {
            "Unknown command" => false,
            _ => true,
        }
    }

    fn execute_command(&mut self, c: Command, history: &mut RedditHistory) {
        match &c.base_command[..] {
            "l" => self.authorize_client(),
            "r" => self.switch_page(history, &c),
            "v" => self.show_posts(history, &c),
            "x" => self.exit_client(),
            "u" => self.not_implemented(),
            "c" => self.not_implemented(),
            "q" => self.not_implemented(),
            _ => (),
        }
    }

    pub fn authorize_client(&mut self) {
        let o_client = authorize_user(self.client_config.auth_time);
        self.o_client = o_client;
    }

    fn switch_page(&mut self, history: &mut RedditHistory, command: &Command) {
        display_message("Which page to visit? /<page_type>/<page>");
        display_message("To visit a subreddit try: /r/rust");
        let target_page = expect_input();
        let split_target_page: Vec<&str> = target_page.split("/").collect();
        if split_target_page.len() <= 2 {
            display_system_message(
                &format!("Invalid visit-page command: {}", target_page),
                String::from("ERROR"),
            );
            display_message("Invalid page to visit.");
            return;
        }
        history.set_target_page(split_target_page[1], split_target_page[2]);
        self.show_posts(history, command);
    }

    fn not_implemented(&self) {
        display_message("Function not implemented yet");
    }

    fn exit_client(&mut self) {
        display_system_message("Exiting Client", "TRACE".to_string());
        self.is_exiting = true;
    }

    fn show_posts(&mut self, history: &RedditHistory, command: &Command) {
        let mut posts: Vec<RedditPost> =
            self.get_subreddit_posts(&history.current_page, history.page_limit);
        let posts_string: String = posts.into_iter().map(|p| p.pretty_string()).collect();
        display_message(&posts_string[..]);
    }

    pub fn get_subreddit_posts(&mut self, subreddit: &str, post_amount: usize) -> Vec<RedditPost> {
        let string_response = curl_site(subreddit, post_amount);
        let posts: SerdeValue = serde_json::from_str(&string_response).unwrap();
        let mut posts_decon: Vec<RedditPost> = Vec::new();
        for n in 0..post_amount {
            posts_decon.push(RedditPost {
                id: strip_serde_string(posts["data"]["children"][n]["data"]["id"].to_string()),
                subreddit: strip_serde_string(
                    posts["data"]["children"][n]["data"]["subreddit"].to_string(),
                ),
                title: strip_serde_string(
                    posts["data"]["children"][n]["data"]["title"].to_string(),
                ),
                ups: strip_serde_string(posts["data"]["children"][n]["data"]["ups"].to_string()),
                score: strip_serde_string(
                    posts["data"]["children"][n]["data"]["score"].to_string(),
                ),
                gilded: 0,
                link_flair_text: strip_serde_string(
                    posts["data"]["children"][n]["data"]["link_flair_text"].to_string(),
                ),
                author: strip_serde_string(
                    posts["data"]["children"][n]["data"]["author"].to_string(),
                ),
                permalink: strip_serde_string(
                    posts["data"]["children"][n]["data"]["permalink"].to_string(),
                ),
                url: strip_serde_string(posts["data"]["children"][n]["data"]["url"].to_string()),
            });
            let reddit_post_object = format!("{:?}", posts_decon[n]);
            display_system_message(&reddit_post_object[..], "TRACE".to_string());
        }
        posts_decon
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

fn strip_serde_string(val: String) -> String {
    let s = val.to_string();
    if s != "null" {
        s.replace("\"", "")
    } else {
        String::from("")
    }
}
