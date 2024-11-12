use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::stela::Hero;

/// Some grand information.
#[derive(Debug, Deserialize, Serialize)]
pub struct SectionHero {
    /// The hero to show.
    pub hero: Rc<Hero>,
}
