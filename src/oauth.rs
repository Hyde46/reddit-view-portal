extern crate rand;
//Concurrency stuff
use std::io::Read;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use tiny_http::{Response, Server};

use serde::{Deserialize, Serialize};

use curl::easy::{Easy, List};

#[derive(Debug)]
enum OAuthState {
    IDLE,
    AUTHORIZED,
    ERROR,
}

#[derive(Serialize, Deserialize, Debug)]
struct OAuthToken {
    access_token: String,
    token_type: String,
    expires_in: usize,
    scope: String,
    refresh_token: String,
}

#[derive(Debug)]
pub struct OAuthClient {
    client_id: String,
    client_state: String,
    authorization_link: String,
    auth_state: OAuthState,
    oauth_token: Option<OAuthToken>,
    pub error_state: String,
    pub code: String,
}

struct AuthBox {
    has_error: bool,
    error_msg: String,
    code: String,
    state: String,
}

impl OAuthClient {
    pub fn new() -> OAuthClient {
        build_oauth_client()
    }
    pub fn get_access_token(&self) -> String {
        if let token = self.oauth_token.as_ref().unwrap() {
            return token.access_token.clone();
        }
        "".to_string()
    }
}

fn build_oauth_client() -> OAuthClient {
    let client_state = generate_random_string(10);
    let client_id = "7tMofTv8Ip3-Ig".to_string();
    let authorization_link = format!( "https://www.reddit.com/api/v1/authorize?client_id={}&response_type=code&state={}&redirect_uri=http%3A%2F%2F127.0.0.1:8000&duration=permanent&scope=identity",client_id,client_state);
    OAuthClient {
        client_id,
        client_state,
        oauth_token: None,
        authorization_link,
        auth_state: OAuthState::IDLE,
        error_state: "Initialized".to_string(),
        code: "".to_string(),
    }
}

pub fn curl_site(subreddit: &str, amount: usize, before: &str, after: &str) -> String {
    let mut limit = amount;
    if limit == 0 {
        limit = 1;
    }
    let user_agent_header = "User-Agent: RVP/0.1 by Gitrog_Frog";
    let mut easy = Easy::new();
    let reddit_base_url = format!(
        "https://www.reddit.com{}/.json?limit={}&after={}&before={}",
        subreddit, limit, after, before
    );
    easy.url(&reddit_base_url).unwrap();
    easy.useragent(user_agent_header).unwrap();

    let mut return_data: Vec<String> = Vec::new();
    let mut html: String = String::new();
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                html = String::from_utf8(Vec::from(data)).unwrap();
                return_data.push(html.clone());
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    };
    return_data.join("")
}

pub fn request_site(token: String, url: String) {
    println!("token:{} url:{}", token, url);

    let mut list = List::new();
    let data_header = format!("Authorization: bearer {}", token);
    println!("data header: {}", data_header);
    list.append(&data_header.to_string()).unwrap();

    let user_agent_header = "User-Agent: RVP/0.1 by Gitrog_Frog";
    let mut easy = Easy::new();
    easy.url(&url).unwrap();
    easy.http_headers(list).unwrap();
    easy.useragent(user_agent_header).unwrap();

    let mut html: String = String::new();
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                html = String::from_utf8(Vec::from(data)).unwrap();
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    };
}

pub fn authorize_user(wait_time: usize) -> OAuthClient {
    println!("Logging in...");
    let mut oauth_client = build_oauth_client();
    if does_access_token_exist() {
        println!("Client already authorized");
        use std::fs;
        let access_token_serialized: String = fs::read_to_string("./access_token.rvp").unwrap();
        let access_token: OAuthToken = serde_json::from_str(&access_token_serialized).unwrap();
        oauth_client = OAuthClient {
            client_id: oauth_client.client_id,
            client_state: oauth_client.client_state,
            oauth_token: Some(access_token),
            authorization_link: oauth_client.authorization_link,
            auth_state: OAuthState::AUTHORIZED,
            error_state: "".to_string(),
            code: "".to_string(),
        }
    } else {
        oauth_client = authorize_client(oauth_client, wait_time);
        println!("Done!");
    }
    oauth_client
}

fn authorize_client(oauth_client: OAuthClient, wait_time: usize) -> OAuthClient {
    if !webbrowser::open(&oauth_client.authorization_link).is_ok() {
        println!("Could not open web browser");
    }
    let final_response =
        Response::from_string("Authentication complete. You may close this window.");

    let (tx_authentication, rx) = mpsc::channel();
    let tx_countdown = mpsc::Sender::clone(&tx_authentication);

    thread::spawn(move || {
        let server = Server::http("127.0.0.1:8000").unwrap();
        for request in server.incoming_requests() {
            let request_url = request.url().to_string().clone();
            let parameter_string: Vec<&str> = request_url.split("/?").collect();
            if parameter_string.len() <= 1 {
                continue;
            };
            let parameters: Vec<&str> = parameter_string[1].split('&').collect();
            // Expect state and code parameters
            if parameters.len() != 2 {
                let auth_box = AuthBox {
                    has_error: true,
                    error_msg: "Unexpected response from reddit api".to_string(),
                    code: "".to_string(),
                    state: "".to_string(),
                };
                tx_authentication.send(auth_box);
            } else {
                let state: Vec<&str> = parameters[0].split('=').collect();
                let code: Vec<&str> = parameters[1].split('=').collect();
                let auth_box = AuthBox {
                    has_error: false,
                    error_msg: "".to_string(),
                    code: code[1].to_string(),
                    state: state[1].to_string(),
                };
                tx_authentication.send(auth_box).unwrap();
            }
        }
        drop(server);
    });
    thread::spawn(move || {
        for passed_seconds in 0..wait_time {
            thread::sleep(Duration::from_secs(1));
        }
        let auth_box = AuthBox {
            has_error: true,
            error_msg: "Reached timeout. User did not authorize usage of RPV in time".to_string(),
            code: "".to_string(),
            state: "".to_string(),
        };
        println!("Timeout during authentication");
        tx_countdown.send(auth_box).unwrap();
    });
    //print!("{}[2J", 27 as char);
    let auth_box = rx.recv().unwrap();
    println!("Now waiting for access token.");

    let data_field_string = format!(
        "grant_type=authorization_code&code={}&redirect_uri=http://127.0.0.1:8000",
        auth_box.code
    );
    println!("Datafield: {}", data_field_string);
    let mut data_field = data_field_string.as_bytes();
    let mut list = List::new();
    let data_header = "Authorization: Basic N3RNb2ZUdjhJcDMtSWc6";
    list.append(data_header).unwrap();

    let user_agent_header = "User-Agent: RVP/0.1 by Gitrog_Frog";
    let mut easy = Easy::new();
    easy.url("https://www.reddit.com/api/v1/access_token")
        .unwrap();
    easy.http_headers(list).unwrap();
    easy.post(true).unwrap();
    easy.useragent(user_agent_header).unwrap();
    easy.post_field_size(data_field.len() as u64).unwrap();

    let mut html: String = String::new();
    {
        let mut transfer = easy.transfer();
        transfer
            .read_function(|buf| Ok(data_field.read(buf).unwrap_or(0)))
            .unwrap();
        transfer
            .write_function(|data| {
                html = String::from_utf8(Vec::from(data)).unwrap();
                Ok(data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    };
    let oauth_token: OAuthToken = serde_json::from_str(&html).unwrap();

    // Handle authentication response
    if !auth_box.has_error {
        if auth_box.state == oauth_client.client_state {
            save_token(&oauth_token);
            OAuthClient {
                client_id: oauth_client.client_id,
                client_state: oauth_client.client_state,
                oauth_token: Some(oauth_token),
                authorization_link: oauth_client.authorization_link,
                auth_state: OAuthState::AUTHORIZED,
                error_state: "".to_string(),
                code: auth_box.code,
            }
        } else {
            OAuthClient {
                client_id: oauth_client.client_id,
                client_state: oauth_client.client_state,
                oauth_token: oauth_client.oauth_token,
                authorization_link: oauth_client.authorization_link,
                auth_state: OAuthState::ERROR,
                error_state: "Return code is not the same. There is some tampering happening."
                    .to_string(),
                code: auth_box.code,
            }
        }
    } else {
        println!("Error: {}", auth_box.error_msg);
        OAuthClient {
            client_id: oauth_client.client_id,
            client_state: oauth_client.client_state,
            oauth_token: oauth_client.oauth_token,
            authorization_link: oauth_client.authorization_link,
            auth_state: OAuthState::ERROR,
            error_state: auth_box.error_msg,
            code: oauth_client.code,
        }
    }
}

fn does_access_token_exist() -> bool {
    use std::path::Path;
    Path::new("./access_token.rvp").exists()
}

fn save_token(token: &OAuthToken) {
    let serialized_token = serde_json::to_string(&token).unwrap();
    use std::fs;
    use std::fs::File;
    use std::io::prelude::*;
    if does_access_token_exist() {
        fs::remove_file("access_token.rvp").expect("Could not remove file");
    }
    let mut file = File::create("access_token.rvp").expect("Unable to create file");
    file.write_all(serialized_token.as_bytes())
        .expect("Unable to write access token");
}

fn generate_random_string(n: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789";
    let mut rng = rand::thread_rng();
    let random_state: String = (0..n)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    random_state
}
