use std::io;

struct RedditHistory {
    current_page_type: String,
    current_page: String,
    page_type_history: Vec<String>,
    page_history: Vec<String>,
    base_url: String,
    oauth_base_url: String,
}

impl RedditHistory {
    pub fn new() -> RedditHistory {
        RedditHistory {
            current_page_type: "/r".to_string(),
            current_page: "/rust".to_string(),
            page_type_history: Vec::new(),
            page_history: Vec::new(),
            base_url: "https://www.reddit.com".to_string(),
            oauth_base_url: "https://oauth.reddit.com".to_string(),
        }
    }
}

/* LOG LEVELS
    "TRACE",
    "DEBUG",
    "INFO",
    "WARN",
    "ERROR",
    "FATAl",
    "OFF",
*/
pub struct RVPUI {
    history: RedditHistory,
    loglevel: String,
}

impl RVPUI {

    pub fn new(loglevel: String) -> RVPUI {
        let hist = RedditHistory::new();
        RVPUI {
            history: hist,
            loglevel: loglevel
        }
    }

    pub fn set_loglevel(&mut self, loglevel: String) {
        self.loglevel = loglevel;
    }

    pub fn print_welcome_message(&self){
        self.display_system_message("Starting up RVP", String::from("TRACE"));
        println!("Reddit View Portal");
    }

    pub fn expect_command(&self) -> String{
        self.display_message("[Command] Waiting for command");
        self.display_message("Log in (login/l)\nVisist subreddit (subreddit/r)");
        RVPUI::expect_input()
    }

    fn expect_input() -> String {
        let mut dialog_answer = String::new();

        io::stdin()
            .read_line(&mut dialog_answer)
            .expect("Failed to read line");
        let trimmed = (&dialog_answer[..]).trim();
        trimmed.to_string()
    }

    fn display_message(&self, m: &str){
        println!("{}",m);
    }

    fn display_system_message(&self, m: &str, min_log_level: String) {
        if self.loglevel == min_log_level {
            self.display_message(m);
        } else {
            if min_log_level == "TRACE" || min_log_level == "UNKNOWN"{
                return;
            }
            let next_log_level = RVPUI::decrease_log_level(min_log_level);
            self.display_system_message(m,next_log_level);
        }
    }

    fn decrease_log_level(l: String) -> String{
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
   
}