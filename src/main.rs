use std::sync::Arc;
use druid::{AppLauncher, WindowDesc};
use im::Vector;

mod page;
mod client;
mod delegate;
mod state;

use delegate::Delegate;
use crate::page::master::build_ui;
use crate::state::data::{AppState, AppViewState, AuthState};
use crate::state::friend::FriendState;
use crate::state::header::HeaderState;

#[tokio::main]
async fn main() {
    let initial_state = AppState {
        auth: AuthState {
            email: "".into(),
            password: "".into(),
            display_message: "Please enter your credentials".into(),
            is_logged_in: false,
        },
        friends: FriendState {
            friends: Arc::new(Vector::new()),
        },
        app_state: AppViewState::Main,
        settings: HeaderState{},
    };

    let main_window = WindowDesc::new(build_ui())
        .title("Todo Tutorial")
        .window_size((1080.0, 1080.0));

    AppLauncher::with_window(main_window)
        .delegate(Delegate {})
        .launch(initial_state)
        .expect("Failed to launch application");
}
