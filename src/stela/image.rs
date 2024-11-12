use std::rc::Rc;

use serde::{Deserialize, Serialize};

/// Image ID, and other useful info.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Image {
    /// Aspect ratio.
    ///
    /// width / height
    pub aspect: Option<f32>,
    /// Original image width.
    pub width: Option<f32>,
    /// Original image height.
    pub height: Option<f32>,
    /// Cloudflare image storage id.
    ///
    /// This will _usually_ be a UUID, but it's not guaranteed to be.
    pub id: Rc<str>,
}
