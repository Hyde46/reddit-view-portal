#[derive(Debug)]
pub struct RedditPost {
    pub id: String,
    pub subreddit: String,
    pub title: String,
    pub ups: usize,
    pub gilded: usize,
    pub link_flair_text: String,
    pub author: String,
    pub permalink: String,
    pub url: String,
}
