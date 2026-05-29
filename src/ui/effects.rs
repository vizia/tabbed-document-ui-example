use crate::ui::events::{DocumentWorkflowEvent, OpenMode, ResourceEvent};
use crate::ui::model::{AppLanguage, DocumentKind, UiModel};
use crate::worker;
use std::path::{Path, PathBuf};
use vizia::prelude::*;

pub fn build_model(cx: &mut Context, app: UiModel) {
    app.build(cx);
}

pub fn pick_file() -> Option<PathBuf> {
    rfd::FileDialog::new().pick_file()
}

pub fn pick_folder() -> Option<PathBuf> {
    rfd::FileDialog::new().pick_folder()
}

pub fn emit_document_workflow_event(cx: &mut EventContext, event: DocumentWorkflowEvent) {
    cx.emit(event);
}

pub fn spawn_create_document(
    cx: &EventContext<'_>,
    file_path: PathBuf,
    doc_type: DocumentKind,
    open_mode: OpenMode,
) {
    let path_for_task = file_path.clone();
    let path_for_result = file_path;

    cx.add_task(
        Task::new(move |_| {
            let path_for_task = path_for_task.clone();
            let path_for_result = path_for_result.clone();
            async move {
                worker::create_new_file(path_for_task, doc_type).await?;
                Ok::<PathBuf, String>(path_for_result)
            }
        })
        .name("create-document")
        .on_result(move |result, proxy| match result {
            TaskResult::Completed(path) => {
                let _ = proxy.emit(DocumentWorkflowEvent::OpenPathRequested { path, open_mode });
            }
            TaskResult::Error(err) => {
                let _ = proxy.emit(DocumentWorkflowEvent::OpenPathFailed(err));
            }
            TaskResult::Timeout | TaskResult::Cancelled | TaskResult::Disconnected { .. } => {}
        }),
    );
}

pub fn spawn_open_document(cx: &EventContext<'_>, path: PathBuf, doc_id: u64, open_mode: OpenMode) {
    let path_for_task = path.clone();

    cx.add_task(
        Task::new(move |_| {
            let path_for_result = path_for_task.clone();
            async move {
                let document =
                    worker::load_document_from_path(path_for_result.clone(), doc_id).await?;
                let image_bytes = if matches!(document.kind, DocumentKind::Image) {
                    Some(
                        tokio::fs::read(&path_for_result)
                            .await
                            .map_err(|e| e.to_string())?,
                    )
                } else {
                    None
                };
                Ok::<(PathBuf, _, Option<Vec<u8>>), String>((
                    path_for_result,
                    document,
                    image_bytes,
                ))
            }
        })
        .name("open-document")
        .on_result(move |result, proxy| match result {
            TaskResult::Completed((path, document, image_bytes)) => {
                let _ = proxy.emit(DocumentWorkflowEvent::OpenPathSucceeded {
                    path,
                    document,
                    image_bytes,
                    open_mode,
                });
            }
            TaskResult::Error(err) => {
                let _ = proxy.emit(DocumentWorkflowEvent::OpenPathFailed(err));
            }
            TaskResult::Timeout | TaskResult::Cancelled | TaskResult::Disconnected { .. } => {}
        }),
    );
}

pub fn spawn_save_text_document(cx: &EventContext<'_>, path: PathBuf, content: String) {
    cx.add_task(
        Task::new(move |_| {
            let path_for_task = path.clone();
            let content_for_task = content.clone();
            async move { worker::save_text_document(path_for_task, content_for_task).await }
        })
        .name("save-text-document")
        .on_result(|result, proxy| match result {
            TaskResult::Completed(_) => {}
            TaskResult::Error(err) => {
                let _ = proxy.emit(DocumentWorkflowEvent::SaveTextFailed(err));
            }
            TaskResult::Timeout | TaskResult::Cancelled | TaskResult::Disconnected { .. } => {}
        }),
    );
}

pub fn register_startup_images(cx: &mut Context, app: &UiModel) {
    for document in app.documents.get_untracked() {
        if matches!(document.kind, DocumentKind::Image)
            && let Ok(image_bytes) = std::fs::read(&document.path)
        {
            cx.emit(ResourceEvent::CacheImageResource(
                document.path,
                image_bytes,
            ));
        }
    }
}

pub fn apply_startup_locale(cx: &mut Context, language: AppLanguage) {
    let locale = match language {
        AppLanguage::English => langid!("en-US"),
        AppLanguage::Spanish => langid!("es-ES"),
    };
    cx.emit(EnvironmentEvent::SetLocale(locale));
}

pub fn register_image_resource(
    cx: &mut EventContext,
    path: &Path,
    image_bytes: &[u8],
    open_mode: OpenMode,
) {
    let retention = if matches!(open_mode, OpenMode::ReplaceActiveNewTab) {
        ImageRetentionPolicy::Forever
    } else {
        ImageRetentionPolicy::DropWhenNoObservers
    };
    cx.add_image_encoded(&path.display().to_string(), image_bytes, retention);
}

pub fn register_persistent_image_resource(cx: &mut EventContext, path: &Path, image_bytes: &[u8]) {
    cx.add_image_encoded(
        &path.display().to_string(),
        image_bytes,
        ImageRetentionPolicy::Forever,
    );
}
