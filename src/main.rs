use druid::{AppLauncher, WindowDesc};
use page::auth::build_auth_ui;

mod page;
mod client;
mod delegate;
mod state;

use delegate::Delegate;
use crate::state::data::AuthState;

#[tokio::main]
async fn main() {
    let initial_state = AuthState {
        email: "".into(),
        login: "".into(),
        display_message: "Please enter your credentials".into(),
        is_logged_in: false,
    };

    let main_window = WindowDesc::new(build_auth_ui())
        .title("Todo Tutorial")
        .window_size((400.0, 400.0));

    AppLauncher::with_window(main_window)
        .delegate(Delegate {})
        .launch(initial_state)
        .expect("Failed to launch application");
}
