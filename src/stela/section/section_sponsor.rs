use serde::{Deserialize, Serialize};

use crate::stela::Motion;

/// Ads. Gotta make money.
#[derive(Debug, Deserialize, Serialize)]
pub struct SectionSponsor {
    /// The word "Sponsor".
    pub sponsor_text: String,
    /// Company/brand being sponosored.
    pub name: String,
    /// Primary text.
    pub text: String,
    /// Call-to-action
    pub motions: Vec<Motion>,
}
