extern crate tiny_http;
mod error;
mod oauth;
mod rvp_api;

use rvp_api::RVPClient;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Reddit portal View");
    println!("Authorizing with reddit. Press Any Key to Continue...");

    let mut dialog_answer = String::new();

    io::stdin()
        .read_line(&mut dialog_answer)
        .expect("Failed to read line");

    let mut rvp: RVPClient = RVPClient::new();
    rvp.get_profile_info();
    rvp.get_subreddit_posts("magicTCG", 2);
    Ok(())
}
