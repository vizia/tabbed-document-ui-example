use crate::ui::model::{Document, DocumentId, TabKind, TabState};
use std::collections::HashMap;
use std::path::Path;

pub(crate) fn title_for_path(path: &Path) -> String {
    path.file_name()
        .and_then(|f| f.to_str())
        .map(std::string::ToString::to_string)
        .unwrap_or_else(|| path.display().to_string())
}

pub(crate) fn tabs_reference_document(tabs: &[TabState], doc_id: DocumentId) -> bool {
    tabs.iter().any(|tab| {
        if let TabKind::Document(document) = &tab.kind {
            document.id == doc_id
        } else {
            false
        }
    })
}

pub(crate) fn refresh_document_tab_titles_for(tabs: &mut [TabState], documents: &[Document]) {
    let mut base_counts: HashMap<String, usize> = HashMap::new();

    for doc in documents {
        let base = title_for_path(&doc.path);
        *base_counts.entry(base).or_insert(0) += 1;
    }

    for tab in tabs {
        if let TabKind::Document(document_state) = &mut tab.kind {
            if let Some(doc) = documents.iter().find(|doc| doc.id == document_state.id) {
                let base = title_for_path(&doc.path);
                let duplicate = base_counts.get(&base).copied().unwrap_or(0) > 1;
                if duplicate {
                    let parent = doc
                        .path
                        .parent()
                        .and_then(|p| p.file_name())
                        .and_then(|p| p.to_str())
                        .unwrap_or("?");
                    tab.title = format!("{parent}/{base}");
                } else {
                    tab.title = base;
                }
                document_state.title = tab.title.clone();
            }
        }
    }
}
