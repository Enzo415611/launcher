use iced::Theme;

#[derive(Clone)]
pub struct UiState {
    pub current_page: Page,
    pub current_theme: Theme,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            current_page: Page::Auth,
            current_theme: Theme::TokyoNight,
        }
    }
}

#[derive(Clone)]
pub enum Page {
    Auth,
    Launcher,
    Config,
}
