use druid::{AppDelegate, Command, DelegateCtx, Env, Handled, Selector, Target};
use crate::client::auth::LoginReply;
use crate::data::AuthState;

const LOGIN_RESPONSE: Selector<Result<LoginReply, String>> = Selector::new("login-response");

pub struct Delegate;

impl AppDelegate<AuthState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AuthState,
        _env: &Env,
    ) -> Handled {
        if cmd.is(LOGIN_RESPONSE) {
            data.is_logged_in = true;
            data.display_message = "u are login".to_string();
            Handled::Yes
        } else {
            println!("cmd forwarded: {:?}", cmd);
            Handled::No
        }
    }
}
