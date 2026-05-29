mod documents;
mod drafts;
mod resources;
mod settings;
mod tabs;
mod workflow;

use crate::ui::model::AppLanguage;
use crate::ui::model::UiModel;
use vizia::prelude::*;

impl Model for UiModel {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        self.handle_tab_events(cx, event);
        self.handle_settings_events(cx, event);
        self.handle_draft_events(cx, event);
        self.handle_document_events(cx, event);
        self.handle_workflow_events(cx, event);
        self.handle_resource_events(cx, event);

        if self.selected_language.get_untracked().is_none() {
            self.selected_language
                .set(Some(match self.settings.get_untracked().language {
                    AppLanguage::English => 0,
                    AppLanguage::Spanish => 1,
                }));
        }

        if self.active_tab_id.get_untracked().is_none() && !self.tabs.get_untracked().is_empty() {
            let first_tab_id = self.tabs.get_untracked()[0].id;
            self.activate_tab(first_tab_id);
        }
    }
}
