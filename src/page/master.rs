use druid::{Widget, WidgetExt};
use druid::widget::Either;
use crate::page::auth::build_auth_ui;
use crate::page::main_page::build_main_ui;
use crate::page::setting::build_settings_page;
use crate::state::data::{AppState, AppViewState};

pub fn build_ui() -> impl Widget<AppState> {
    Either::new(
        |data: &AppState, _| data.app_state == AppViewState::Auth,
        build_auth_ui().lens(AppState::auth),
        Either::new(
            |data: &AppState, _| data.app_state == AppViewState::Main,
            build_main_ui(),
            build_settings_page(),
        ),
    )
}
