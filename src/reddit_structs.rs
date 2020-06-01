#[derive(Debug)]
pub struct RedditPost {
    pub render_id: String,
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
    pub before: String,
    pub after: String
}

impl RedditPost {
    pub fn pretty_string(&self) -> String {
        if self.link_flair_text == "" {
            format!(
                "({}) [{}] - {} in /r/{} | Score:{}\n",
                self.render_id, self.title, self.author, self.subreddit, self.score
            )
        } else {
            format!(
                "({}) [{}]({}) - {} in /r/{} | Score:{}\n",
                self.render_id, self.title, self.link_flair_text, self.author, self.subreddit, self.score
            )
        }
    }
}
