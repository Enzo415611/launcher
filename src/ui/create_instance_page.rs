use iced::{
    Alignment, Widget,
    widget::{column, container, pick_list, row, text_input},
};

use crate::{
    events::{
        instance::{CreateInstanceEvents, InstanceEvents},
        ui::UiEvents,
        update::AppEvent,
    },
    launcher::instance::InstanceLoader,
    state::{app_state::AppState, ui::Page},
    ui::components::rbutton::r_button,
};

// pub name: String,
// pub loader: InstanceLoader,
// pub loader_version: String,
// pub mc_version: String,

impl AppState {
    pub fn create_instance_page(&self) -> impl Widget<AppEvent> {
        container(column![
            row![r_button("X").on_press(AppEvent::Ui(UiEvents::SwapPage(Page::Launcher)))]
                .align_y(Alignment::Start),
            text_input("Instance Name", &self.instance.create_instance.name).on_input(|i| {
                AppEvent::Instance(InstanceEvents::CreateInstanceEvents(
                    CreateInstanceEvents::Name(i),
                ))
            }),
            pick_list(
                self.instance.create_instance.loader.clone(),
                InstanceLoader::ALL,
                InstanceLoader::to_string
            )
            .on_select(|l| AppEvent::Instance(InstanceEvents::CreateInstanceEvents(
                CreateInstanceEvents::Loader(l)
            ))),
        ])
    }
}
