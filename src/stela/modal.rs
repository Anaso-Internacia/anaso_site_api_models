use serde::{Deserialize, Serialize};

use crate::stela::Section;

/// Pop-up section over page content.
#[derive(Debug, Deserialize, Serialize)]
pub struct Modal {
    /// What to display.
    pub section: Section,
}
