use serde::{Deserialize, Serialize};

use crate::stela::{Image, VisualMotion};

/// Some grand information.
#[derive(Debug, Deserialize, Serialize)]
pub struct Hero {
    /// Image to show on light theme.
    pub primary_image_light: Option<Image>,
    /// Image to show on dark theme.
    pub primary_image_dark: Option<Image>,
    /// Text to show when image isn't available or still loading.
    pub primary_image_fallback_text: Option<String>,

    /// Background to show on light theme.
    pub background_image_light: Option<Image>,
    /// Background to show on dark theme.
    pub background_image_dark: Option<Image>,

    /// Header text.
    pub title: Option<String>,
    /// Subheader text.
    pub description: Option<String>,

    /// Buttons user can click.
    pub motions: Vec<VisualMotion>,
}
