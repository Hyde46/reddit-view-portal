use crate::reddit_structs::*;
use crate::rvp_api::Command;
use std::io;

/* LOG LEVELS
    "TRACE",
    "DEBUG",
    "INFO",
    "WARN",
    "ERROR",
    "FATAL",
    "OFF",
*/
static LOGLEVEL: &'static str = "TRACE";

pub struct RedditHistory {
    pub current_page: String,
    pub page_limit: usize,
    page_history: Vec<String>,
    base_url: String,
    oauth_base_url: String,
    current_ui_type: String,
    ui_type_history: Vec<String>,
    pub current_post_after_hash: String,
    post_after_hash_history: Vec<String>,
    pub current_post_before_hash: String,
    post_before_hash_history: Vec<String>,
    pub threads_in_view: Vec<RedditPost>,
}

impl RedditHistory {
    pub fn new() -> RedditHistory {
        RedditHistory {
            current_page: "/r/popular".to_string(),
            page_history: Vec::new(),
            base_url: "https://www.reddit.com".to_string(),
            oauth_base_url: "https://oauth.reddit.com".to_string(),
            page_limit: 10,
            current_ui_type: "SUBREDDIT".to_string(),
            ui_type_history: Vec::new(),
            current_post_after_hash: "".to_string(),
            post_after_hash_history: Vec::new(),
            current_post_before_hash: "".to_string(),
            post_before_hash_history: Vec::new(),
            threads_in_view: Vec::new(),
        }
    }
    pub fn set_target_page(&mut self, page: &str) {
        display_system_message(&format!("Visiting page {}", page), String::from("DEBUG"));
        self.page_history.push(self.current_page.clone());
        self.current_page = page.to_string();
        self.ui_type_history.push(self.current_ui_type.clone());
    }
    pub fn set_post_hash(&mut self, before: String, after: String) {
        self.post_before_hash_history.push(before.to_string());
        self.post_after_hash_history.push(after.to_string());
        self.current_post_before_hash = before.to_string();
        self.current_post_after_hash = after.to_string();
    }
    pub fn set_threads_in_view(&mut self, threads_in_view: Vec<RedditPost>) {
        self.threads_in_view = threads_in_view;
    }
    pub fn get_threads_in_view_size(&self) -> usize {
        self.threads_in_view.len()
    }
    pub fn get_pretty_pagetype(&self) -> String {
        match &self.current_ui_type[..] {
            "BASE" => " Main Page".to_string(),
            "SUBREDDIT" => " In Subreddit".to_string(),
            _ => "".to_string(),
        }
    }
}

pub fn print_welcome_message() {
    display_system_message("Starting up RVP", String::from("TRACE"));
    println!("Reddit View Portal");
}

pub fn display_status(history: &RedditHistory) {
    let status = format!(
        "[Currently {}{}]",
        history.current_page,
        history.get_pretty_pagetype()
    );
    display_message(&status);
}

pub fn display_help() {
    display_message(
        "Available commands: 
- Switch subreddit (subreddit/r)
- View posts on subreddit (posts/v)
- Show comments of post (comments/p <number>)
- Show next posts (next/n)
- Show previous posts (before/b)
- Create post (create/c)
- Search User (user/u)
- Login (login/l)
- Logout (logout/q)
- Help (help/h)
- Exit Reddit View Portal (exit/x)",
    );
}

pub fn expect_command() -> Command {
    display_message("Waiting for command...");
    let input = expect_input().to_ascii_lowercase();
    let mut split_command: Vec<&str> = input.split(" ").collect();
    // Pad command if no parameter is supplied
    if split_command.len() == 1 {
        split_command.push("");
    }
    Command {
        base_command: split_command[0].to_string(),
        parameter: split_command[1].to_string(),
    }
}

pub fn expect_input() -> String {
    let mut dialog_answer = String::new();
    io::stdin()
        .read_line(&mut dialog_answer)
        .expect("Failed to read line");
    (&dialog_answer[..]).trim().to_string()
}

pub fn display_message(m: &str) {
    println!("{}", m);
}

fn display_log_message(m: &str, log_level: String) {
    println!("[{}] {}", log_level, m);
}

pub fn display_system_message(m: &str, min_log_level: String) {
    find_log_level(m, min_log_level.clone(), min_log_level.clone());
}

fn find_log_level(m: &str, current_log: String, min_log_level: String) {
    if LOGLEVEL == current_log {
        display_log_message(m, min_log_level);
    } else {
        if current_log == "TRACE" || current_log == "UNKNOWN" {
            return;
        }
        let next_log_level = decrease_log_level(current_log);
        display_system_message(m, next_log_level);
    }
}

fn decrease_log_level(l: String) -> String {
    match &l[..] {
        "OFF" => String::from("FATAL"),
        "FATAL" => String::from("ERROR"),
        "ERROR" => String::from("WARN"),
        "WARN" => String::from("INFO"),
        "INFO" => String::from("DEBUG"),
        "DEBUG" => String::from("TRACE"),
        _ => String::from("UNKNOWN"),
    }
}
