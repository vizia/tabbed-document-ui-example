use crate::ui::events::SettingsEvent;
use crate::ui::model::UiModel;
use vizia::prelude::*;

impl UiModel {
    pub(crate) fn handle_settings_events(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|settings_event: &SettingsEvent, _| match settings_event {
            SettingsEvent::ChangeLanguage(language) => self.set_language(cx, *language),
            SettingsEvent::ToggleHomeOnStartup => self.toggle_open_home_on_startup(),
        });
    }
}
