pub const DEFAULT_DOCUMENT_SIDEBAR_WIDTH: f32 = 300.0;
pub const MIN_DOCUMENT_SIDEBAR_WIDTH: f32 = 120.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DocumentUiState {
    pub sidebar_width: f32,
    pub scroll_x: f32,
    pub scroll_y: f32,
}

impl Default for DocumentUiState {
    fn default() -> Self {
        Self {
            sidebar_width: DEFAULT_DOCUMENT_SIDEBAR_WIDTH,
            scroll_x: 0.0,
            scroll_y: 0.0,
        }
    }
}
