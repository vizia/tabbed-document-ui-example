pub fn show_open_error(err: &str) {
    let title = if err.starts_with("Unsupported file type") {
        "Unsupported file"
    } else {
        "Open failed"
    };

    show_message(title, err, rfd::MessageLevel::Error);
}

fn show_message(title: &str, description: &str, level: rfd::MessageLevel) {
    let _ = rfd::MessageDialog::new()
        .set_level(level)
        .set_title(title)
        .set_description(description)
        .show();
}
