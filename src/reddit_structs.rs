#[derive(Debug)]
pub struct RedditPost {
    pub id: String,
    pub subreddit: String,
    pub title: String,
    pub ups: String,
    pub score: String,
    pub gilded: usize,
    pub link_flair_text: String,
    pub author: String,
    pub permalink: String,
    pub url: String,
}

impl RedditPost {
    pub fn pretty_string(&self) -> String {
        if self.link_flair_text == "" {
            format!(
                "[{}] - {} in /r/{} | Score:{}\n\n",
                self.title, self.author, self.subreddit, self.score
            )
        } else {
            format!(
                "[{}]({}) - {} in /r/{} | Score:{}\n\n",
                self.title, self.link_flair_text, self.author, self.subreddit, self.score
            )
        }
    }
}
