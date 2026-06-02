use crate::ui::model::DocumentKind;
use crate::ui::model::TabKind;
use crate::ui::model::UiModel;
use crate::ui::views;
use vizia::prelude::*;

pub fn build_app_shell(cx: &mut Context, app: UiModel) {
    let spawned_document_windows = Signal::new(Vec::<u64>::new());
    Binding::new(cx, app.detached_windows, move |cx| {
        let requested_windows = app.detached_windows.get();
        let spawned_ids = spawned_document_windows.get_untracked();

        for window in requested_windows
            .iter()
            .cloned()
            .filter(|window| !spawned_ids.contains(&window.id))
        {
            let spawned_for_close = spawned_document_windows;
            let detached_tab = window.tab.clone();
            let window_id = window.id;
            let window_title = detached_tab.title.clone();

            spawned_document_windows.update(|ids| ids.push(window_id));

            Window::new(cx, move |cx| match &detached_tab.kind {
                TabKind::Home => views::build_home(cx, app.clone()),
                TabKind::New(_) => views::build_new_tab(cx, app.clone(), detached_tab.id),
                TabKind::Document(document_ref) => {
                    if let Some(document) = app.document_by_id(document_ref.id) {
                        let path_display = document.path.display().to_string();
                        match document.kind {
                            DocumentKind::Text => views::build_text_document_page(
                                cx,
                                app.clone(),
                                document.id,
                                path_display,
                            ),
                            DocumentKind::Image => views::build_image_document_page(
                                cx,
                                app.clone(),
                                document.id,
                                path_display,
                                document.image_state,
                            ),
                        }
                    } else {
                        Label::new(cx, "Document no longer available.");
                    }
                }
            })
            .title(window_title)
            .inner_size((900, 700))
            .on_close(move |_cx| {
                spawned_for_close.update(|ids| ids.retain(|id| *id != window_id));
                app.clear_detached_window(window_id);
            });
        }
    });

    VStack::new(cx, move |cx| {
        views::toolbar(cx, app.clone());
        views::content_host(cx, app.clone());
        views::status_bar(cx, app.clone());
    })
    .size(Stretch(1.0));
}
