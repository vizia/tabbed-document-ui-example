use crate::ui::events::{TabContextEvent, TabEvent};
use crate::ui::model::UiModel;
use crate::ui::model::{DocumentKind, TabKind};
use crate::ui::views::empty_state_page::build_no_tabs_open_state;
use crate::ui::views::home_page::build_home;
use crate::ui::views::image_document_page::build_image_document_page;
use crate::ui::views::new_document_page::build_new_tab;
use crate::ui::views::text_document_page::build_text_document_page;
use vizia::prelude::*;

pub fn content_host(cx: &mut Context, app: UiModel) {
    let has_tabs = app.tabs.map(|tabs| !tabs.is_empty());
    let selected_index = app.tabs.map(move |tabs| {
        app.active_tab_id
            .get()
            .and_then(|id| tabs.iter().position(|tab| tab.id == id))
            .unwrap_or(0)
    });

    VStack::new(cx, move |cx| {
        VStack::new(cx, |cx| {
            build_no_tabs_open_state(cx);
        })
        .display(has_tabs.map(|has_tabs| {
            if *has_tabs {
                Display::None
            } else {
                Display::Flex
            }
        }))
        .size(Stretch(1.0));

        TabView::new(cx, app.tabs, move |_cx, _index, tab| {
            let tab_id = tab.id;
            let title = tab.title.clone();
            let tab_kind = tab.kind.clone();
            let tab_kind_for_content = tab_kind.clone();
            let closeable = tab.closable;

            TabPair::new(
                {
                    let title_for_header = title.clone();
                    move |cx| {
                        let title_for_row = title_for_header.clone();

                        Label::new(cx, title_for_row.clone())
                            .hoverable(false)
                            .class("tab-title-label");
                    }
                },
                move |cx| match &tab_kind_for_content {
                    TabKind::Home => build_home(cx, app.clone()),
                    TabKind::New(_) => build_new_tab(cx, app.clone(), tab_id),
                    TabKind::Document(document) => {
                        if let Some(doc) = app.document_by_id(document.id) {
                            let path_display = doc.path.display().to_string();
                            match doc.kind {
                                DocumentKind::Text => {
                                    build_text_document_page(cx, app.clone(), doc.id, path_display)
                                }
                                DocumentKind::Image => build_image_document_page(
                                    cx,
                                    app.clone(),
                                    doc.id,
                                    path_display,
                                    doc.image_state,
                                ),
                            }
                        }
                    }
                },
            )
            .menu({
                let tab_kind_for_menu = tab_kind.clone();
                move |cx| {
                    let tab_kind_for_menu = tab_kind_for_menu.clone();
                    Menu::new(cx, Placement::Cursor, true, move |cx| {
                        if matches!(tab_kind_for_menu, TabKind::Document(_) | TabKind::New(_)) {
                            MenuButton::new(
                                cx,
                                move |cx| cx.emit(TabContextEvent::Duplicate(tab_id)),
                                |cx| Label::new(cx, "Duplicate Tab"),
                            );
                        }

                        MenuButton::new(
                            cx,
                            move |cx| cx.emit(TabContextEvent::CloseOthers(tab_id)),
                            |cx| Label::new(cx, "Close Other Tabs"),
                        );

                        Divider::new(cx);
                        MenuButton::new(
                            cx,
                            move |cx| cx.emit(TabContextEvent::DisplayInWindow(tab_id)),
                            |cx| Label::new(cx, "Open In Separate Window"),
                        );
                    })
                }
            })
            .closeable(closeable)
        })
        .with_selected(selected_index)
        .on_select({
            move |cx, index| {
                if let Some(tab) = app.tabs.get_untracked().get(index).cloned() {
                    cx.emit(TabEvent::ActivateTab(tab.id));
                }
            }
        })
        .on_close({
            move |cx, index| {
                if let Some(tab) = app.tabs.get_untracked().get(index).cloned() {
                    if tab.closable {
                        cx.emit(TabEvent::CloseTab(tab.id));
                    }
                }
            }
        })
        .display(has_tabs.map(|has_tabs| {
            if *has_tabs {
                Display::Flex
            } else {
                Display::None
            }
        }))
        .size(Stretch(1.0));
    })
    .class("content-host")
    .size(Stretch(1.0));
}
