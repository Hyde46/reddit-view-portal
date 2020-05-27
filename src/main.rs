extern crate tiny_http;
mod error;
mod oauth;
mod reddit_structs;
mod rvp_api;
mod rvp_ui;

use rvp_api::RVPClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rvp: RVPClient = RVPClient::new();
    rvp.run();
    Ok(())
}
