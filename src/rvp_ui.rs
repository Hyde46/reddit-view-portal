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
    pub current_page_type: String,
    pub current_page: String,
    pub page_limit: usize,
    page_type_history: Vec<String>,
    page_history: Vec<String>,
    base_url: String,
    oauth_base_url: String,
}

impl RedditHistory {
    pub fn new() -> RedditHistory {
        RedditHistory {
            current_page_type: "r".to_string(),
            current_page: "rust".to_string(),
            page_type_history: Vec::new(),
            page_history: Vec::new(),
            base_url: "https://www.reddit.com".to_string(),
            oauth_base_url: "https://oauth.reddit.com".to_string(),
            page_limit: 10,
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
    }
}

pub fn print_welcome_message() {
    display_system_message("Starting up RVP", String::from("TRACE"));
    println!("Reddit View Portal");
}

pub fn display_status(history: &RedditHistory) {
    let status = format!(
        "[Currently /{}/{}]",
        history.current_page_type, history.current_page
    );
    display_message(&status);
}

pub fn expect_command() -> String {
    display_message("Waiting for command...");
    display_message("-Log in (login/l)\n-Switch subreddit (subreddit/r)\n-View posts on subreddit (posts/v)\n-Exit (exit/x)");
    expect_input()
}

pub fn expect_input() -> String {
    let mut dialog_answer = String::new();

    io::stdin()
        .read_line(&mut dialog_answer)
        .expect("Failed to read line");
    let trimmed = (&dialog_answer[..]).trim();
    trimmed.to_string()
}

pub fn display_message(m: &str) {
    println!("{}", m);
}

pub fn display_log_message(m: &str, log_level: String) {
    println!("[{}] {}", log_level, m);
}

fn display_system_message(m: &str, min_log_level: String) {
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
