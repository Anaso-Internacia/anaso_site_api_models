use serde::{Deserialize, Serialize};

/// Website meta tags.
#[derive(Debug, Deserialize, Serialize)]
pub struct SocialData {
    /// Key-value pairs.
    ///
    /// Example:
    /// ```text
    /// [
    ///     ("og:title", "a/Hejmo"),
    ///     ("og:description", "La socia retejo nur por Esperantistoj"),
    ///     ("og:url", "https://ana.so/a/Hejmo"),
    /// ]
    /// ```
    pub tags: Vec<(String, String)>,
}
