use crate::ui::model::DocumentId;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum DocumentKind {
    Text,
    Image,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImageState {
    pub width: u32,
    pub height: u32,
    pub last_click: Option<(f32, f32)>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Document {
    pub id: DocumentId,
    pub kind: DocumentKind,
    pub path: PathBuf,
    pub text_content: Option<String>,
    pub image_state: Option<ImageState>,
}
