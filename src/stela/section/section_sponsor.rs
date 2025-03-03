use serde::{Deserialize, Serialize};

use crate::stela::Motion;

use super::Section;

/// Ads. Gotta make money.
#[derive(bon::Builder, Debug, Deserialize, Serialize)]
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

impl From<SectionSponsor> for Section {
    fn from(value: SectionSponsor) -> Self {
        Section::Sponsor(value.into())
    }
}
