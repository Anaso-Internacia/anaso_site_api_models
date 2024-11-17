use serde::{Deserialize, Serialize};

use crate::stela::{Image, Motion, VisualMotion};

/// User-generated content.
#[derive(Debug, Deserialize, Serialize)]
pub struct SectionPost {
    /// Header text.
    pub title: Option<String>,
    /// Main post image.
    pub image: Option<Image>,
    /// Blurred background image.
    pub background: Option<Image>,
    /// Show a pin icon.
    pub is_pinned: Option<bool>,
    /// Post content in HTML form.
    pub body_html: Option<String>,
    /// What to do when post is clicked.
    pub motion: Option<Motion>,
    /// Top-left motions.
    pub motions_tl: Vec<VisualMotion>,
    /// Top-right motions.
    pub motions_tr: Vec<VisualMotion>,
    /// Bottom-right motions.
    pub motions_br: Vec<VisualMotion>,
    /// Bottom-left motions.
    pub motions_bl: Vec<VisualMotion>,
}
