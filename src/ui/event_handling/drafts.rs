use crate::ui::events::DraftEvent;
use crate::ui::model::UiModel;
use vizia::prelude::*;

impl UiModel {
    pub(crate) fn handle_draft_events(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|draft_event: &DraftEvent, _| match draft_event {
            DraftEvent::RenameDraft(tab_id, name) => self.update_new_name(*tab_id, name.clone()),
            DraftEvent::ChooseDraftType(tab_id, selected) => self.set_new_type(*tab_id, *selected),
            DraftEvent::ChooseDraftDirectory(tab_id) => self.pick_new_directory(cx, *tab_id),
            DraftEvent::SubmitDraft(tab_id) => self.submit_new_tab(cx, *tab_id),
        });
    }
}
