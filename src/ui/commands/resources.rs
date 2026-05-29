use crate::ui::effects;
use std::path::PathBuf;
use vizia::prelude::*;

pub fn cache_image_resource(cx: &mut EventContext, path: PathBuf, image_bytes: Vec<u8>) {
    effects::register_persistent_image_resource(cx, path.as_path(), &image_bytes);
}
