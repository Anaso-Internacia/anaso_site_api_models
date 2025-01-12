use serde::{Deserialize, Serialize};

/// Website meta tags.
#[derive(Debug, Deserialize, Serialize)]
pub struct SocialData {
    /// `name`, `og:title`, `twitter:title`
    pub title: Option<String>,
    /// `description`, `og:description`, `twitter:description`
    pub description: Option<String>,
    /// `image`, `og:image`, `twitter:image`
    pub image: Option<String>,
    /// `og:url`
    pub url: Option<String>,
    /// `twitter:card`
    pub twitter_card: Option<String>,
}
