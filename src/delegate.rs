use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Selector, Target};
use crate::client::auth::LoginReply;
use crate::state::data::{AppState, AppViewState};

const LOGIN_RESPONSE: Selector<Result<LoginReply, String>> = Selector::new("login-response");
pub const SWITCH_TO_SETTINGS: Selector<AppViewState> = Selector::new("switch_to_settings");


pub struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if cmd.is(LOGIN_RESPONSE) {
            data.auth.is_logged_in = true;
            data.auth.display_message = "u are login".to_string();
            data.app_state = AppViewState::Main;
            Handled::Yes
        } else if cmd.is(SWITCH_TO_SETTINGS) {
            data.app_state = AppViewState::Setting;
            Handled::Yes
        } else {
            println!("cmd forwarded: {:?}", cmd);
            Handled::No
        }
    }
}
