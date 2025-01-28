use serde::{Deserialize, Serialize};

use crate::stela::{Image, Motion, VisualMotion};

/// Representation of data and buttons on the top/side navbars.
#[derive(Debug, Deserialize, Serialize)]
pub struct Navbar {
    /// Motion for when left icon image is selected.
    pub left_side_motion: Motion,
    /// Primary image to show on left side of navbar.
    pub left_side_icon_image: Image,
    /// Dim text to show next to primary icon image.
    pub left_side_secondary_text: Option<String>,
    /// Motion for when search bar is clicked.
    pub search_motion: Option<Motion>,
    /// Text inside the search bar.
    pub search_text: Option<String>,
    /// Buttons on right side of navbar.
    pub right_side_motions: Vec<VisualMotion>,
    /// Buttons in side nav.
    pub side_motions: Vec<VisualMotion>,
}
