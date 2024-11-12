use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::stela::VisualMotion;

/// Fill something out and submit.
#[derive(Debug, Deserialize, Serialize)]
pub struct SectionForm {
    /// Primary text at top.
    pub header: Option<Rc<str>>,
    /// Secondary text under header.
    pub subheader: Option<Rc<str>>,
    /// Individual input fields.
    pub inputs: Vec<FormInput>,
}

/// Individual input field.
#[derive(Debug, Deserialize, Serialize)]
pub struct FormInput {
    /// Human-readable name.
    pub title: Option<Rc<str>>,
    /// What kind of field is it.
    pub variant: FormInputVariant,
}

/// What kind of input it is with needed extra info.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum FormInputVariant {
    /// Toggle on or off.
    Checkbox(Rc<FormInputCheckbox>),
    /// Upload an image.
    Image(Rc<FormInputImage>),
    /// Type full markdown.
    Markdown(Rc<FormInputMarkdown>),
    /// A list of motions.
    Motions(Rc<FormInputMotions>),
    /// Select from multiple options.
    Radio(Rc<FormInputRadio>),
    /// A smaller form with a title inside the full form.
    Subsection(Rc<FormInputSubsection>),
    /// Type text.
    Text(Rc<FormInputText>),
    /// Unknown form input.
    #[default]
    #[serde(other)]
    Unknown,
}

/// A smaller form with a title inside the full form.
#[derive(Debug, Deserialize, Serialize)]
pub struct FormInputSubsection {
    /// Individual input fields.
    pub inputs: Vec<FormInput>,
}

/// This is a text field
#[derive(Debug, Deserialize, Serialize)]
pub struct FormInputText {
    /// What to put in form-data for the API.
    pub name: Option<Rc<str>>,
    /// Minimum character count.
    pub length_min: Option<i32>,
    /// Maximum character count.
    pub length_max: Option<i32>,
    /// Add x-system converter button.
    pub esperanto: bool,
    /// Filter out certain letters.
    pub filter: Option<TextFilter>,
}

bitflags::bitflags! {
    /// Allow-list of kinds of characters.
    #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
    pub struct TextFilter: u32 {
        /// `A-Z` ascii
        const ALPHA_ASCII = 1;
        /// `A-Z` esperanto (no x, includes hats)
        const ALPHA_EO = 1 << 1;
        /// `0-9`
        const NUMERIC = 1 << 2;
        /// `-`
        const DASH = 1 << 3;
        /// `_`
        const UNDERSCORE = 1 << 4;
        /// `.`
        const PERIOD = 1 << 5;
        /// ` `
        const SPACE = 1 << 6;
    }
}

/// Upload an image.
#[derive(Debug, Deserialize, Serialize)]
pub struct FormInputImage {
    /// What to put in form-data for the API.
    pub name: Option<Rc<str>>,
    /// How to show the image after upload.
    pub preview_style: ImagePreviewStyle,
}

/// How to show an image after upload.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq)]
pub enum ImagePreviewStyle {
    /// Full-width of form.
    LargeRectangle,
    /// Small rectangle image next to upload button.
    ThumbnailRect,
    /// Small circle image next to upload button.
    ThumbnailCircle,
    /// Unknown style.
    #[default]
    #[serde(other)]
    Unknown,
}

/// Write large body text as markdown.
#[derive(Debug, Deserialize, Serialize)]
pub struct FormInputMarkdown {
    /// What to put in form-data for the API.
    pub name: Rc<str>,
    /// Minimum character count.
    pub length_min: Option<i32>,
    /// Maximum character count.
    pub length_max: Option<i32>,
}

/// Select from multiple options.
///
/// Can only select one.
#[derive(Debug, Deserialize, Serialize)]
pub struct FormInputRadio {
    /// What to put in form-data for the API.
    pub name: Rc<str>,
    /// Individual selectable options.
    pub options: Vec<RadioButton>,
}

/// An individual radio button.
#[derive(Debug, Deserialize, Serialize)]
pub struct RadioButton {
    /// What to put in form-data for the API.
    pub value: Rc<str>,
    /// Human-readable text for option.
    pub title: Option<Rc<str>>,
}

/// Toggle on or off.
#[derive(Debug, Deserialize, Serialize)]
pub struct FormInputCheckbox {
    /// What to put in form-data for the API.
    pub name: Rc<str>,
    /// Should it start checked.
    pub default_checked: Option<bool>,
}

/// A list of motions.
#[derive(Debug, Deserialize, Serialize)]
pub struct FormInputMotions {
    /// Show vertically instead of horizontally.
    pub vertical_list: Option<bool>,
    /// The list of motions to show.
    pub motions: Vec<VisualMotion>,
}
