use crate::ui::model::{Document, DocumentKind, ImageState};
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use tokio::task;

pub fn load_document_from_path_blocking(path: &Path, id: u64) -> Result<Document, String> {
    let extension = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|ext| ext.to_ascii_lowercase())
        .unwrap_or_default();

    match extension.as_str() {
        "txt" | "md" | "rs" | "toml" | "json" | "log" => {
            let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
            Ok(Document {
                id,
                kind: DocumentKind::Text,
                path: path.to_path_buf(),
                text_content: Some(text),
                image_state: None,
            })
        }
        "bmp" | "png" | "jpg" | "jpeg" => {
            let img = image::open(path).map_err(|e| e.to_string())?;
            Ok(Document {
                id,
                kind: DocumentKind::Image,
                path: path.to_path_buf(),
                text_content: None,
                image_state: Some(ImageState {
                    width: img.width(),
                    height: img.height(),
                    last_click: None,
                }),
            })
        }
        _ => Err(format!("Unsupported file type: {}", path.display())),
    }
}

pub async fn load_document_from_path(path: PathBuf, id: u64) -> Result<Document, String> {
    let extension = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|ext| ext.to_ascii_lowercase())
        .unwrap_or_default();

    match extension.as_str() {
        "txt" | "md" | "rs" | "toml" | "json" | "log" => {
            let text = tokio::fs::read_to_string(&path)
                .await
                .map_err(|e| e.to_string())?;
            Ok(Document {
                id,
                kind: DocumentKind::Text,
                path,
                text_content: Some(text),
                image_state: None,
            })
        }
        "bmp" | "png" | "jpg" | "jpeg" => task::spawn_blocking(move || {
            let img = image::open(&path).map_err(|e| e.to_string())?;
            Ok(Document {
                id,
                kind: DocumentKind::Image,
                path,
                text_content: None,
                image_state: Some(ImageState {
                    width: img.width(),
                    height: img.height(),
                    last_click: None,
                }),
            })
        })
        .await
        .map_err(|err| format!("Image load task failed: {err}"))?,
        _ => Err(format!("Unsupported file type: {}", path.display())),
    }
}

pub async fn create_new_file(path: PathBuf, kind: DocumentKind) -> Result<(), String> {
    match kind {
        DocumentKind::Text => tokio::fs::write(path, "").await.map_err(|e| e.to_string()),
        DocumentKind::Image => task::spawn_blocking(move || {
            let mut image = image::RgbImage::new(1, 1);
            image.put_pixel(0, 0, image::Rgb([255, 255, 255]));
            image.save(path).map_err(|e| e.to_string())
        })
        .await
        .map_err(|err| format!("Image write task failed: {err}"))?,
    }
}

pub async fn save_text_document(path: PathBuf, content: String) -> Result<(), String> {
    tokio::fs::write(path, content)
        .await
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[tokio::test]
    async fn saves_text_document_to_disk() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("tabbed-ui-save-test-{unique}.txt"));

        save_text_document(path.clone(), "hello world".to_string())
            .await
            .expect("save should succeed");

        let content = std::fs::read_to_string(&path).expect("saved file should be readable");
        assert_eq!(content, "hello world");

        let _ = std::fs::remove_file(path);
    }

    #[tokio::test]
    async fn create_and_load_text_document_behavior() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("tabbed-ui-behavior-{unique}.txt"));

        create_new_file(path.clone(), DocumentKind::Text)
            .await
            .expect("text file should be created");
        save_text_document(path.clone(), "integration content".to_string())
            .await
            .expect("text should be saved");

        let loaded = load_document_from_path(path.clone(), 1)
            .await
            .expect("document should load");
        assert!(matches!(loaded.kind, DocumentKind::Text));
        assert_eq!(loaded.text_content.as_deref(), Some("integration content"));

        let _ = std::fs::remove_file(path);
    }

    #[tokio::test]
    async fn unsupported_extension_reports_error_behavior() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("tabbed-ui-behavior-{unique}.xyz"));

        std::fs::write(&path, "noop").expect("temp file should be created");

        let err = load_document_from_path(path.clone(), 1)
            .await
            .expect_err("load should fail");
        assert!(err.starts_with("Unsupported file type:"));

        let _ = std::fs::remove_file(path);
    }
}
