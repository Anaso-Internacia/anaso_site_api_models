use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::stela::Hero;

use super::Section;

/// Some grand information.
#[derive(bon::Builder, Debug, Deserialize, Serialize)]
pub struct SectionHero {
    /// The hero to show.
    pub hero: Arc<Hero>,
}

impl From<SectionHero> for Section {
    fn from(value: SectionHero) -> Self {
        Section::Hero(value.into())
    }
}
