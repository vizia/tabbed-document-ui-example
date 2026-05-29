use super::UiModel;
use crate::ui::model::AppLanguage;
use vizia::prelude::*;

impl UiModel {
    pub fn set_language(&self, cx: &mut EventContext, language: AppLanguage) {
        let mut settings = self.settings.get_untracked();
        settings.language = language;
        self.settings.set(settings);

        let (locale, idx) = match language {
            AppLanguage::English => (langid!("en-US"), Some(0)),
            AppLanguage::Spanish => (langid!("es-ES"), Some(1)),
        };

        self.selected_language.set(idx);
        cx.emit(EnvironmentEvent::SetLocale(locale));
        self.persist_state();
    }

    pub fn toggle_open_home_on_startup(&self) {
        let mut settings = self.settings.get_untracked();
        settings.open_home_on_startup = !settings.open_home_on_startup;
        self.settings.set(settings);
        self.persist_state();
    }
}
