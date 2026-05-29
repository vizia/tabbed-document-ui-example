use crate::ui::commands::startup;
use crate::ui::model::UiModel;
use vizia::prelude::*;

pub fn bootstrap_app(cx: &mut Context) -> UiModel {
    load_resources(cx);

    cx.emit(EnvironmentEvent::SetThemeMode(ThemeMode::LightMode));

    let app = UiModel::new(cx);
    startup::build_model(cx, app);
    startup::register_startup_images(cx, &app);
    startup::apply_startup_locale(cx, app.settings.get_untracked().language);

    app
}

fn load_resources(cx: &mut Context) {
    cx.add_stylesheet(include_style!("resources/stylesheets/theme.css"))
        .expect("Failed to load theme stylesheet");

    cx.add_translation(
        langid!("en-US"),
        include_str!("../../resources/translations/en-US/strings.ftl"),
    )
    .expect("Failed to load en-US translation");

    cx.add_translation(
        langid!("es-ES"),
        include_str!("../../resources/translations/es-ES/strings.ftl"),
    )
    .expect("Failed to load es-ES translation");
}
