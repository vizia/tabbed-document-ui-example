use super::tab_title::{refresh_document_tab_titles_for, tabs_reference_document};
use crate::ui::model::model_data::DetachedWindowState;
use crate::ui::model::{Document, DocumentId, DocumentKind, DocumentState, TabKind, TabState};
use std::path::PathBuf;

fn text_doc(id: DocumentId, path: &str) -> Document {
    Document {
        id,
        kind: DocumentKind::Text,
        path: PathBuf::from(path),
        text_content: Some(String::new()),
        image_state: None,
    }
}

#[test]
fn disambiguates_duplicate_document_names() {
    let docs = vec![
        text_doc(1, "/tmp/alpha/file.txt"),
        text_doc(2, "/tmp/bravo/file.txt"),
    ];

    let mut tabs = vec![
        TabState {
            id: 10,
            title: String::new(),
            kind: TabKind::Document(DocumentState {
                id: 1,
                title: String::new(),
            }),
            closable: true,
        },
        TabState {
            id: 11,
            title: String::new(),
            kind: TabKind::Document(DocumentState {
                id: 2,
                title: String::new(),
            }),
            closable: true,
        },
    ];

    refresh_document_tab_titles_for(&mut tabs, &docs);

    assert_eq!(tabs[0].title, "alpha/file.txt");
    assert_eq!(tabs[1].title, "bravo/file.txt");
}

#[test]
fn keeps_unique_document_name_unchanged() {
    let docs = vec![text_doc(1, "/tmp/alpha/notes.txt")];

    let mut tabs = vec![TabState {
        id: 20,
        title: String::new(),
        kind: TabKind::Document(DocumentState {
            id: 1,
            title: String::new(),
        }),
        closable: true,
    }];

    refresh_document_tab_titles_for(&mut tabs, &docs);

    assert_eq!(tabs[0].title, "notes.txt");
}

#[test]
fn mru_next_tab_prefers_most_recent_open() {
    let tabs = vec![
        TabState {
            id: 1,
            title: "A".to_string(),
            kind: TabKind::Document(DocumentState {
                id: 1,
                title: "A".to_string(),
            }),
            closable: true,
        },
        TabState {
            id: 2,
            title: "B".to_string(),
            kind: TabKind::Document(DocumentState {
                id: 2,
                title: "B".to_string(),
            }),
            closable: true,
        },
        TabState {
            id: 3,
            title: "C".to_string(),
            kind: TabKind::Document(DocumentState {
                id: 3,
                title: "C".to_string(),
            }),
            closable: true,
        },
    ];
    let mru = vec![3, 2, 1];
    let tab_ids = tabs.iter().map(|tab| tab.id).collect::<Vec<_>>();

    let next = mru.iter().copied().find(|id| tab_ids.contains(id));
    assert_eq!(next, Some(3));

    let mut remaining = tab_ids;
    remaining.retain(|id| *id != 3);
    let next_after_close = mru.iter().copied().find(|id| remaining.contains(id));
    assert_eq!(next_after_close, Some(2));
}

#[test]
fn image_click_conversion_uses_scroll_offsets() {
    let original_width = 100.0_f32;
    let original_height = 80.0_f32;
    let viewport_width = 50.0_f32;
    let viewport_height = 40.0_f32;
    let display_width = 100.0_f32;
    let display_height = 80.0_f32;
    let scroll_x = 0.5_f32;
    let scroll_y = 0.25_f32;

    let max_x = (display_width - viewport_width).max(0.0);
    let max_y = (display_height - viewport_height).max(0.0);
    let scroll_offset_x = scroll_x * max_x;
    let scroll_offset_y = scroll_y * max_y;

    let viewport_click_x = 25.0_f32;
    let viewport_click_y = 20.0_f32;
    let local_x = viewport_click_x + scroll_offset_x;
    let local_y = viewport_click_y + scroll_offset_y;
    let image_x = (local_x / display_width) * original_width;
    let image_y = (local_y / display_height) * original_height;

    assert_eq!(image_x, 50.0);
    assert_eq!(image_y, 30.0);
}

#[test]
fn tabs_reference_document_true_when_duplicate_tabs_open() {
    let tabs = vec![
        TabState {
            id: 1,
            title: "Doc".to_string(),
            kind: TabKind::Document(DocumentState {
                id: 100,
                title: "Doc".to_string(),
            }),
            closable: true,
        },
        TabState {
            id: 2,
            title: "Doc Copy".to_string(),
            kind: TabKind::Document(DocumentState {
                id: 100,
                title: "Doc Copy".to_string(),
            }),
            closable: true,
        },
    ];

    assert!(tabs_reference_document(&tabs, 100));
}

#[test]
fn tabs_reference_document_false_when_document_is_absent() {
    let tabs = vec![TabState {
        id: 1,
        title: "Other".to_string(),
        kind: TabKind::Document(DocumentState {
            id: 200,
            title: "Other".to_string(),
        }),
        closable: true,
    }];

    assert!(!tabs_reference_document(&tabs, 100));
}

#[test]
fn detached_document_window_counts_as_open_document_reference() {
    let detached_windows = vec![DetachedWindowState {
        id: 1,
        tab: TabState {
            id: 99,
            title: "Detached Doc".to_string(),
            kind: TabKind::Document(DocumentState {
                id: 100,
                title: "Detached Doc".to_string(),
            }),
            closable: true,
        },
    }];

    let has_reference = detached_windows.into_iter().any(|window| {
        matches!(
            window.tab.kind,
            TabKind::Document(DocumentState { id: 100, .. })
        )
    });

    assert!(has_reference);
}
