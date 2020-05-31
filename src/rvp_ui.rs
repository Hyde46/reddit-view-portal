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
static LOGLEVEL: &'static str = "OFF";

pub struct RedditHistory {
    pub current_page_type: String,
    pub current_page: String,
    pub page_limit: usize,
    page_type_history: Vec<String>,
    page_history: Vec<String>,
    base_url: String,
    oauth_base_url: String,
    current_ui_type: String,
    ui_type_history: Vec<String>,
}

impl RedditHistory {
    pub fn new() -> RedditHistory {
        RedditHistory {
            current_page_type: "r".to_string(),
            current_page: "popular".to_string(),
            page_type_history: Vec::new(),
            page_history: Vec::new(),
            base_url: "https://www.reddit.com".to_string(),
            oauth_base_url: "https://oauth.reddit.com".to_string(),
            page_limit: 10,
            current_ui_type: "SUBREDDIT".to_string(),
            ui_type_history: Vec::new(),
        }
    }
    pub fn set_target_page(&mut self, page_type: &str, page: &str) {
        display_system_message(
            &format!("Visiting page /{}/{}", page_type, page),
            String::from("DEBUG"),
        );
        self.page_type_history.push(self.current_page_type.clone());
        self.page_history.push(self.current_page.clone());
        self.current_page_type = page_type.to_string();
        self.current_page = page.to_string();
        self.ui_type_history.push(self.current_ui_type.clone());
        self.current_ui_type = match page_type {
            "r" => "SUBREDDIT".to_string(),
            _ => "BASE".to_string(),
        }
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
        "[Currently /{}/{}{}]",
        history.current_page_type,
        history.current_page,
        history.get_pretty_pagetype()
    );
    display_message(&status);
}

pub fn expect_command() -> Command {
    display_message("Waiting for command...");
    display_message(
        "- Switch subreddit (subreddit/r)
- View posts on subreddit (posts/v)
- Create post (create/c)
- Search User (user/u)
- Login (login/l)
- Logout (logout/q)
- Exit Reddit View Portal (exit/x)",
    );
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
