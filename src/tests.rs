use crate::{error::TwitterError, scraper::Scraper};

pub async fn get_session() -> Result<Scraper, TwitterError> {
    dotenv::dotenv().ok();
    let mut scraper = Scraper::new().await?;
    let cookie_string = std::env::var("TWITTER_COOKIE_STRING")
        .expect("TWITTER_COOKIE_STRING environment variable not set");
    scraper.set_from_cookie_string(&cookie_string).await?;
    Ok(scraper)
}
