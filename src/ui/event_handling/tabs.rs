use crate::ui::commands::{documents, tabs};
use crate::ui::events::{TabContextEvent, TabEvent};
use crate::ui::model::UiModel;
use vizia::prelude::*;

impl UiModel {
    pub(crate) fn handle_tab_events(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|tab_event: &TabEvent, _| match tab_event {
            TabEvent::RequestHomeTab => self.ensure_home_tab(),
            TabEvent::RequestOpenFile => documents::request_open_file(cx),
            TabEvent::CreateTab => self.add_new_tab(),
            TabEvent::CloseAllTabs => self.close_all_tabs(),
            TabEvent::ActivateTab(tab_id) => self.activate_tab(*tab_id),
            TabEvent::CloseTab(tab_id) => self.close_tab(*tab_id),
        });

        event.map(|context_event: &TabContextEvent, _| match context_event {
            TabContextEvent::Duplicate(tab_id) => self.duplicate_tab(*tab_id),
            TabContextEvent::CloseOthers(tab_id) => self.close_other_tabs(*tab_id),
            TabContextEvent::DisplayInWindow(tab_id) => {
                tabs::display_document_in_window(*self, *tab_id)
            }
        });
    }
}
