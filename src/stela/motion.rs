use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::stela::Modal;

/// Display a motion as a button.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VisualMotion {
    /// Primary button text.
    pub title: Option<Rc<str>>,
    /// Slug for the button icon.
    pub icon: Option<MotionIcon>,
    /// Whether the button should be in a toggled on state initially.
    pub initial_toggle: Option<bool>,
    /// How to display the motion.
    pub variant: MotionVariant,
    /// What color should it be.
    pub color: MotionColor,
    /// The motion to perform when clicked.
    pub motion: Motion,
}

/// Show an icon as part of the motion.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub enum MotionIcon {
    /// Speech bubble.
    Comment,
    /// Heart.
    Like,
    /// Shield.
    Moderation,
    /// Push pin.
    Pin,
    /// Fat arrow pointing right.
    Share,
    /// Toggle.
    Toggle,
    /// Show a fallback icon.
    ///
    /// This doesn't mean don't show an icon. It means show tofu.
    #[default]
    #[serde(other)]
    Unknown,
}

/// How to display a motion.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub enum MotionVariant {
    /// Rectangle with text and icon inside.
    Button,
    /// Like `Button`, but the rectangle is a border.
    ButtonBorder,
    /// Just the text with no background.
    Link,
    /// Like `Link`, but have a button-like rectangle on hover.
    LinkHoverButton,
    /// Like `LinkHoverButton`, but the rectangle is a border
    LinkHoverButtonBorder,
    /// Use fallback.
    #[default]
    #[serde(other)]
    Unknown,
}

/// What color a motion should be.
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
pub enum MotionColor {
    /// Site color.
    Primary,
    /// Secondary site color.
    Secondary,
    /// Black.
    Text,
    /// Fallback.
    #[default]
    #[serde(other)]
    Unknown,
}

/// What to do when interacted with.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum Motion {
    /// Call the `motion_interaction` endpoint and do something with the response.
    ApiCall(Rc<MotionApiCall>),
    /// Navigate to this link.
    Href(Rc<MotionHref>),
    /// Show a share dialogue.
    Share(Rc<MotionShare>),
    /// Submit the form.
    Submit(Rc<MotionSubmit>),
    /// Unrecognized motion.
    #[default]
    #[serde(other)]
    Unknown,
}

/// Call the `motion_interaction` endpoint and do something with the response.
#[derive(Debug, Deserialize, Serialize)]
pub struct MotionApiCall {
    /// If `Some`, change the toggle to this new value.
    pub new_toggle: Option<bool>,
    /// Show a pop-up.
    pub modal: Option<Rc<Modal>>,
    /// Send the user to the given URI.
    pub redirect: Option<Rc<str>>,
}

/// Navigate to this link.
#[derive(Debug, Deserialize, Serialize)]
pub struct MotionHref {
    /// Where to go.
    pub uri: Rc<str>,
    /// target="_blank"
    pub new_tab: Option<bool>,
}

/// Show a share dialogue.
#[derive(Debug, Deserialize, Serialize)]
pub struct MotionShare {
    /// Title to be shared.
    pub title: Option<Rc<str>>,
    /// Text to be shared.
    pub text: Option<Rc<str>>,
    /// Url to be shared.
    pub url: Option<Rc<str>>,
}

/// Submit the form.
#[derive(Debug, Deserialize, Serialize)]
pub struct MotionSubmit {}
