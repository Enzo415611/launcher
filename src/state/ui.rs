#[derive(Clone)]
pub struct UiState {
    pub current_page: Page,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            current_page: Page::Auth,
        }
    }
}

#[derive(Clone)]
pub enum Page {
    Auth,
    Launcher,
}
