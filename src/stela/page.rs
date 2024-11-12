use std::rc::Rc;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnError, VecSkipError};

use crate::stela::{Hero, Sidebar, SocialData, VisualSection};

/// An entire page.
#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct Page {
    /// The title of the page.
    #[serde_as(as = "DefaultOnError")]
    pub title: Option<Rc<str>>,
    /// What language is the page itself in.
    ///
    /// Individual sections can still have their own languages.
    ///
    /// This is an `ISO-639` locale code, such as `en`, `eo`, `es`, `fr`.
    #[serde_as(as = "DefaultOnError")]
    pub lang: Option<Rc<str>>,
    /// Info for the meta tags.
    #[serde_as(as = "DefaultOnError")]
    pub social: Option<Rc<SocialData>>,
    /// How to display things on screen.
    #[serde_as(as = "DefaultOnError")]
    pub layout: Option<PageLayout>,
    /// Display something fancy at the top of the page.
    #[serde_as(as = "DefaultOnError")]
    pub hero: Option<Rc<Hero>>,
    /// Mini sections on the side of the screen.
    #[serde_as(as = "DefaultOnError")]
    pub sidebar: Option<Rc<Sidebar>>,
    /// Individual sections of the page.
    #[serde_as(as = "VecSkipError<_>")]
    #[serde(default)]
    pub sections: Vec<VisualSection>,
}

/// How to display things on screen.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub enum PageLayout {
    /// All sections laid out vertically.
    List,
    /// Each section behind a tab.
    Tabbed,
    /// Fallback.
    #[default]
    #[serde(other)]
    Unknown,
}
