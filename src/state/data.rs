use druid::{Data, Lens};
use crate::state::friend::FriendState;
use crate::state::header::HeaderState;

#[derive(Clone, Data, Lens)]
pub struct AuthState {
    pub email: String,
    pub password: String,
    pub display_message: String,
    pub is_logged_in: bool,
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub auth: AuthState,
    pub friends: FriendState,
    pub app_state: AppViewState,
    pub settings: HeaderState,
}

#[derive(Clone, Data, PartialEq)]
pub enum AppViewState {
    Auth,
    Main,
    Setting,
}

impl AuthState {

}
