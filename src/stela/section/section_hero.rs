use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::stela::Hero;

/// Some grand information.
#[derive(Debug, Deserialize, Serialize)]
pub struct SectionHero {
    /// The hero to show.
    pub hero: Arc<Hero>,
}
