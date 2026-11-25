use iced::{Theme, widget::pane_grid};

#[derive(Clone)]
pub struct UiState {
    pub current_page: Page,
    pub current_theme: Theme,
    pub launcher_grid: pane_grid::State<Panes>,
}

impl Default for UiState {
    fn default() -> Self {
        let (mut state, instances_pane) = pane_grid::State::new(Panes::Instances);
        state.split(pane_grid::Axis::Vertical, instances_pane, Panes::Right);

        Self {
            current_page: Page::Auth,
            current_theme: Theme::TokyoNight,
            launcher_grid: state,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Page {
    Auth,
    Launcher,
    Config,
    CreateInstance,
}

#[derive(Clone)]
pub enum Panes {
    Instances,
    Right,
}
