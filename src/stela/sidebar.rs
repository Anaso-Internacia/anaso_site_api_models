use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::stela::VisualMotion;

/// Mini sections on the side of the screen.
#[derive(Debug, Deserialize, Serialize)]
pub struct Sidebar {
    /// The sections to show.
    pub cards: Vec<SidebarCard>,
}

/// An individual sidebar section.
#[derive(Debug, Deserialize, Serialize)]
pub struct SidebarCard {
    /// Headr text at the top.
    pub title: Arc<str>,
    /// Main text.
    pub body: Arc<str>,
    /// Buttons underneath body.
    pub motions: Vec<VisualMotion>,
}
