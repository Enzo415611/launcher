use iced::{Task, widget::pane_grid};

use crate::{
    events::update::AppEvent,
    launcher::instance::Instance,
    state::{app_state::AppState, ui::Page},
};

#[derive(Debug, Clone)]
pub enum UiEvents {
    PaneResize(pane_grid::ResizeEvent),
    SwapPage(Page),
}

impl AppState {
    pub fn ui_events(&mut self, e: UiEvents) -> Task<AppEvent> {
        match e {
            UiEvents::PaneResize(r) => {
                self.ui_state.launcher_grid.resize(r.split, r.ratio);
                Task::none()
            }
            UiEvents::SwapPage(p) => {
                if p == Page::CreateInstance {
                    self.instance.create_instance = Instance::default();
                }
                self.ui_state.current_page = p;
                Task::none()
            }
        }
    }
}
