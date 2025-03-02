use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::stela::{Image, Modal, VisualMotion};

use super::Section;

/// Fill something out and submit.
#[derive(bon::Builder, Debug, Deserialize, Serialize)]
pub struct SectionForm {
    /// Primary text at top.
    pub header: Option<String>,
    /// Secondary text under header.
    pub subheader: Option<String>,
    /// Which form is this.
    ///
    /// Arbitrary, unstructured data. Provided by API.
    pub form_name: String,
    /// Arbitrary, unstructured data. Provided by API.
    pub extra_data: Option<String>,
    /// Text to show in the `<noscript>` tag.
    pub noscript_text: Option<String>,
    /// Individual input fields.
    pub inputs: Vec<FormInput>,
}

impl From<SectionForm> for Section {
    fn from(value: SectionForm) -> Self {
        Section::Form(value.into())
    }
}

/// What kind of input it is with needed extra info.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum FormInput {
    /// Toggle on or off.
    Checkbox(Arc<FormInputCheckbox>),
    /// Cloudflare Turnstile
    CfTurnstile(Arc<FormInputCfTurnstile>),
    /// Upload image or paste a link.
    Tabs(Arc<FormInputTabs>),
    /// Upload an image.
    Image(Arc<FormInputImage>),
    /// Type full markdown.
    Markdown(Arc<FormInputMarkdown>),
    /// A list of motions.
    Motions(Arc<FormInputMotions>),
    /// Select from multiple options.
    Radio(Arc<FormInputRadio>),
    /// A smaller form with a title inside the full form.
    Subsection(Arc<FormInputSubsection>),
    /// Type text.
    Text(Arc<FormInputText>),
    /// Unknown form input.
    #[default]
    #[serde(other)]
    Unknown,
}

/// Data to pass to the `form_submit()` server function.
#[derive(bon::Builder, Clone, Debug, Deserialize, Serialize)]
pub struct FormCallData {
    /// Which form is this.
    ///
    /// Arbitrary, unstructured data. Provided by API.
    pub form_name: String,
    /// Arbitrary, unstructured data. Provided by API.
    pub extra_data: Option<String>,
    /// The values of the form fields as entered by the user.
    pub fields: HashMap<String, String>,
}

/// Call the `form_submit` endpoint and do something with the response.
#[derive(bon::Builder, Clone, Debug, Default, Deserialize, Serialize)]
pub struct FormResponse {
    /// Show an error.
    pub error: Option<String>,
    /// Show a pop-up.
    pub modal: Option<Arc<Modal>>,
    /// Send the user to the given URI.
    pub redirect: Option<String>,
    /// Hide the form and show this text.
    pub success: Option<String>,
}

/// A smaller form with a title inside the full form.
#[derive(bon::Builder, Debug, Deserialize, Serialize)]
pub struct FormInputSubsection {
    /// Human-readable name.
    pub title: Option<String>,
    /// Individual input fields.
    pub inputs: Vec<FormInput>,
}

impl From<FormInputSubsection> for FormInput {
    fn from(value: FormInputSubsection) -> Self {
        FormInput::Subsection(value.into())
    }
}

/// This is a text field
#[derive(bon::Builder, Debug, Deserialize, Serialize)]
pub struct FormInputText {
    /// Human-readable name.
    pub title: Option<String>,
    /// What to put in form-data for the API.
    pub name: Option<String>,
    /// Value to start with.
    pub initial_value: Option<String>,
    /// Minimum character count.
    pub length_min: Option<i32>,
    /// Maximum character count.
    pub length_max: Option<i32>,
    /// Add x-system converter button.
    pub esperanto: bool,
    /// Filter out certain letters.
    pub filter: Option<TextFilter>,
}

impl From<FormInputText> for FormInput {
    fn from(value: FormInputText) -> Self {
        FormInput::Text(value.into())
    }
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
#[derive(bon::Builder, Debug, Deserialize, Serialize)]
pub struct FormInputImage {
    /// Human-readable name.
    pub title: Option<String>,
    /// What to put in form-data for the API.
    pub name: Option<String>,
    /// Image to start with.
    pub initial_image: Option<Image>,
    /// How to show the image after upload.
    pub preview_style: ImagePreviewStyle,
}

impl From<FormInputImage> for FormInput {
    fn from(value: FormInputImage) -> Self {
        FormInput::Image(value.into())
    }
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
#[derive(bon::Builder, Debug, Deserialize, Serialize)]
pub struct FormInputMarkdown {
    /// Human-readable name.
    pub title: Option<String>,
    /// What to put in form-data for the API.
    pub name: String,
    /// Text to start with.
    pub initial_value: Option<String>,
    /// Minimum character count.
    pub length_min: Option<i32>,
    /// Maximum character count.
    pub length_max: Option<i32>,
}

impl From<FormInputMarkdown> for FormInput {
    fn from(value: FormInputMarkdown) -> Self {
        FormInput::Markdown(value.into())
    }
}

/// Select from multiple options.
///
/// Can only select one.
#[derive(bon::Builder, Debug, Deserialize, Serialize)]
pub struct FormInputRadio {
    /// Human-readable name.
    pub title: Option<String>,
    /// What to put in form-data for the API.
    pub name: String,
    /// Index of initial value.
    pub initial_index: Option<usize>,
    /// Individual selectable options.
    pub options: Vec<RadioButton>,
}

impl From<FormInputRadio> for FormInput {
    fn from(value: FormInputRadio) -> Self {
        FormInput::Radio(value.into())
    }
}

/// An individual radio button.
#[derive(bon::Builder, Debug, Deserialize, Serialize)]
pub struct RadioButton {
    /// What to put in form-data for the API.
    pub value: String,
    /// Human-readable text for option.
    pub title: Option<String>,
}

/// Toggle on or off.
#[derive(bon::Builder, Debug, Deserialize, Serialize)]
pub struct FormInputCheckbox {
    /// Human-readable name.
    pub title: Option<String>,
    /// What to put in form-data for the API.
    pub name: String,
    /// Should it start checked.
    pub default_checked: Option<bool>,
}

impl From<FormInputCheckbox> for FormInput {
    fn from(value: FormInputCheckbox) -> Self {
        FormInput::Checkbox(value.into())
    }
}

/// Cloudflare Turnstile
#[derive(bon::Builder, Debug, Deserialize, Serialize)]
pub struct FormInputCfTurnstile {
    /// Attribute `class`
    pub class: Option<String>,
    /// Attribute `data-sitekey`
    pub sitekey: String,
    /// Attribute `data-response-field-name`
    pub response_field_name: Option<String>,
    /// Attribute `data-size`
    pub size: Option<String>,
    /// Attribute `data-language`
    pub language: Option<String>,
}

impl From<FormInputCfTurnstile> for FormInput {
    fn from(value: FormInputCfTurnstile) -> Self {
        FormInput::CfTurnstile(value.into())
    }
}

/// Tabs of multiple optional inputs
#[derive(bon::Builder, Debug, Deserialize, Serialize)]
pub struct FormInputTabs {
    /// Labeled tabs
    pub tabs: Vec<FormInputTab>,
    /// Start with the tab of this index shown.
    pub initial_index: Option<usize>,
}

impl From<FormInputTabs> for FormInput {
    fn from(value: FormInputTabs) -> Self {
        FormInput::Tabs(value.into())
    }
}

/// Labeled form tab
#[derive(bon::Builder, Debug, Deserialize, Serialize)]
pub struct FormInputTab {
    /// Tab label
    pub title: String,
    /// Input shown when tab selected.
    pub input: FormInput,
}

/// A list of motions.
#[derive(bon::Builder, Debug, Deserialize, Serialize)]
pub struct FormInputMotions {
    /// Show vertically instead of horizontally.
    pub vertical_list: Option<bool>,
    /// The list of motions to show.
    pub motions: Vec<VisualMotion>,
}

impl From<FormInputMotions> for FormInput {
    fn from(value: FormInputMotions) -> Self {
        FormInput::Motions(value.into())
    }
}
