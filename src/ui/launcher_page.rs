use iced::{
    Alignment, Element, Length, Widget,
    widget::{Scrollable, button, column, container, pane_grid, text},
};

use crate::{
    events::update::AppEvent,
    state::{app_state::AppState, ui::Panes},
    ui::components::{instance_button::instance_button, rbutton::r_button},
};

impl AppState {
    pub fn launcher_page(&self) -> impl Widget<AppEvent> {
        let grid = pane_grid(&self.ui_state.launcher_grid, |_, panes, _| {
            pane_grid::Content::new(match panes {
                Panes::Instances => self.instance_list(),
                Panes::Right => self.right_grid(),
            })
        })
        .on_resize(5, AppEvent::PaneResize);

        container(grid)
    }

    fn instance_list(&self) -> Element<'_, AppEvent> {
        container(column![
            r_button("New Instance"),
            Scrollable::new(
                column![].extend(self.instance.instances.iter().map(|i| instance_button(i)),)
            )
        ])
        .boxed()
    }

    fn right_grid(&self) -> Element<'_, AppEvent> {
        let profile_button = if let Some(profile) = self.auth_state.profile.as_ref() {
            Some(r_button(profile.username.as_str()))
        } else {
            None
        };
        container(
            column![profile_button]
                .height(Length::Fill)
                .align_x(Alignment::Start),
        )
        .boxed()
    }
}
