use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::stela::Motion;

/// Ads. Gotta make money.
#[derive(Debug, Deserialize, Serialize)]
pub struct SectionSponsor {
    /// The word "Sponsor".
    pub sponsor_text: Arc<str>,
    /// Company/brand being sponosored.
    pub name: Arc<str>,
    /// Primary text.
    pub text: Arc<str>,
    /// Call-to-action
    pub motions: Vec<Motion>,
}
