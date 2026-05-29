use crate::ui::effects;
use crate::ui::model::{AppLanguage, UiModel};
use vizia::prelude::*;

pub fn build_model(cx: &mut Context, app: UiModel) {
    effects::build_model(cx, app);
}

pub fn register_startup_images(cx: &mut Context, app: &UiModel) {
    effects::register_startup_images(cx, app);
}

pub fn apply_startup_locale(cx: &mut Context, language: AppLanguage) {
    effects::apply_startup_locale(cx, language);
}
