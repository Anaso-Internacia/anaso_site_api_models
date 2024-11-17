use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::stela::{Image, Motion};

/// List of clickable tiles.
#[derive(Debug, Deserialize, Serialize)]
pub struct SectionTiles {
    /// The list of tiles.
    pub tiles: Vec<Tile>,
    /// How to layout the tiles.
    pub layout: TilesLayout,
}

/// Individual clickable tile.
///
/// See [`SectionTiles`].
#[derive(Debug, Deserialize, Serialize)]
pub struct Tile {
    /// Primary text.
    pub header: Option<Arc<str>>,
    /// Secondary text.
    pub subheader: Option<Arc<str>>,
    /// What to do when clicked.
    pub motion: Option<Motion>,
    /// Both thumbnail and background.
    pub image: Option<Image>,
    /// Show text instead of thumbnail.
    pub body_text: Option<Arc<str>>,
}

/// How to layout tiles.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub enum TilesLayout {
    /// Scrolling shelf.
    HorizontalList,
    /// Scrolling list,
    VerticalList,
    /// Grid of tiles.
    Grid,
    /// Fallback.
    #[default]
    #[serde(other)]
    Unknown,
}
