use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::stela::Motion;

/// Ads. Gotta make money.
#[derive(Debug, Deserialize, Serialize)]
pub struct SectionSponsor {
    /// The word "Sponsor".
    pub sponsor_text: Rc<str>,
    /// Company/brand being sponosored.
    pub name: Rc<str>,
    /// Primary text.
    pub text: Rc<str>,
    /// Call-to-action
    pub motions: Vec<Motion>,
}
