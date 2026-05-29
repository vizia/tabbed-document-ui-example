use crate::ui::effects;
use crate::ui::model::{TabId, UiModel};
use vizia::prelude::*;

pub fn pick_draft_directory(model: UiModel, tab_id: TabId) {
    if let Some(directory) = effects::pick_folder() {
        let mut drafts = model.new_tab_drafts.get_untracked();
        let draft = drafts.entry(tab_id).or_default();
        draft.directory = Some(directory);
        model.new_tab_drafts.set(drafts);
    }
}
