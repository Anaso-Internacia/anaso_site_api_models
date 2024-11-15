use std::rc::Rc;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnError};

pub use section_form::*;
pub use section_hero::*;
pub use section_post::*;
pub use section_sponsor::*;
pub use section_tiles::*;

mod section_form;
mod section_hero;
mod section_post;
mod section_sponsor;
mod section_tiles;

/// A blob of UI with some generic display info.
#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VisualSection {
    /// Name to display for the section.
    pub title: Option<Rc<str>>,
    /// Should the section be drawn with a border around it.
    pub bordered: Option<bool>,
    /// The actual section.
    #[serde_as(as = "DefaultOnError")]
    pub section: Section,
}

/// A blob of UI.
///
/// Can be a post, or a shelf of tiles, or various other things.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum Section {
    /// Fill something out and submit.
    Form(Rc<SectionForm>),
    /// Some grand information.
    Hero(Rc<SectionHero>),
    /// User-generated content
    Post(Rc<SectionPost>),
    /// Ads. Gotta make money.
    Sponsor(Rc<SectionSponsor>),
    /// List of clickable tiles.
    Tiles(Rc<SectionTiles>),
    /// Section couldn't be parsed or the type is unknown.
    #[default]
    #[serde(other)]
    Unknown,
}
