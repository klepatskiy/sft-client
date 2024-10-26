use std::error::Error;
use tokio::spawn;
use druid::widget::{Button, Flex, Label, TextBox};
use druid::{Widget, WidgetExt, Target, Selector};
use crate::client::auth::LoginReply;
use crate::client::grpc_login_request;
use crate::state::data::AuthState;

const LOGIN_RESPONSE: Selector<Result<LoginReply, String>> = Selector::new("login-response");

pub fn build_auth_ui() -> impl Widget<AuthState> {
    let email_input = TextBox::new()
        .with_placeholder("Enter your email")
        .lens(AuthState::email)
        .padding(5.0);

    let login_input = TextBox::new()
        .with_placeholder("Enter your login")
        .lens(AuthState::login)
        .padding(5.0);

    let submit_button = Button::new("Submit").on_click(|ctx, data: &mut AuthState, _env| {
        let email = data.email.clone();
        let login = data.login.clone();

        let handle = ctx.get_external_handle();
        spawn(async move {
            let result = async_login(email, login).await;
            let result = result.map_err(|e| e.to_string());
            handle.submit_command(LOGIN_RESPONSE, result, Target::Auto).unwrap();
        });
    });

    let display_label = Label::new(|data: &AuthState, _env: &_| data.display_message.clone())
        .padding(5.0);

    Flex::column()
        .with_child(email_input)
        .with_child(login_input)
        .with_child(submit_button)
        .with_child(display_label)
        .padding(10.0)
}

async fn async_login(email: String, login: String) -> Result<LoginReply, Box<dyn Error>> {
    grpc_login_request(email, login).await
}
