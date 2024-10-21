use druid::widget::{Button, Flex, Label, TextBox};
use druid::{Lens, Widget, WidgetExt, Env, EventCtx, Target, Selector, Command};
use std::thread;
use crate::state::AuthState;
use tokio::runtime::Runtime;
use crate::client::auth::LoginReply;
use crate::client::grpc_login_request;

const LOGIN_RESPONSE: Selector<Result<LoginReply, String>> = Selector::new("login-response");

pub fn build_auth_ui() -> impl Widget<AuthState> {
    // Поле для ввода email
    let email_input = TextBox::new()
        .with_placeholder("Enter your email")
        .lens(AuthState::email)
        .padding(5.0);

    // Поле для ввода логина
    let login_input = TextBox::new()
        .with_placeholder("Enter your login")
        .lens(AuthState::login)
        .padding(5.0);

    // Кнопка отправки
    let submit_button = Button::new("Submit")
        .on_click(|ctx: &mut EventCtx, data: &mut AuthState, _env: &Env| {
            let email = data.email.clone();
            let login = data.login.clone();
            let tx = ctx.get_external_handle();  // Получаем handle для отправки события в основной поток

            // Запускаем gRPC-запрос в отдельном потоке
            thread::spawn(move || {
                let rt = Runtime::new().unwrap();
                let result = rt.block_on(grpc_login_request(email, login));

                // Отправляем результат обратно в основной поток через событие
                match result {
                    Ok(reply) => {
                        tx.submit_command(LOGIN_RESPONSE, Ok(reply), Target::Auto)
                            .expect("Failed to send success response");
                    }
                    Err(e) => {
                        tx.submit_command(LOGIN_RESPONSE, Err(format!("gRPC Error: {}", e)), Target::Auto)
                            .expect("Failed to send error response");
                    }
                }
            });
        })
        .padding(5.0);

    // Метка для отображения сообщений
    let display_label = Label::new(|data: &AuthState, _env: &_| data.display_message.clone())
        .padding(5.0);

    // Вернем UI с помощью Flex
    Flex::column()
        .with_child(email_input)
        .with_child(login_input)
        .with_child(submit_button) // Добавляем кнопку
        .with_child(display_label)
        .padding(10.0)
}

// Функция для обновления состояния на основе ответа от gRPC-запроса
pub fn update_login_response(data: &mut AuthState, result: Result<LoginReply, String>) {
    match result {
        Ok(reply) => {
            if reply.success {
                data.display_message = format!("Welcome, {}!", data.login);
                data.is_logged_in = true;
            } else {
                data.display_message = format!("Login failed: {}", reply.message);
            }
        }
        Err(error_message) => {
            data.display_message = format!("Error: {}", error_message);
        }
    }
}

// Функция для обработки события и обновления UI на основе gRPC ответа
pub fn handle_login_response(ctx: &mut EventCtx, data: &mut AuthState, cmd: &Command) {
    if let Some(result) = cmd.get(LOGIN_RESPONSE) {
        update_login_response(data, result.clone());
        ctx.request_update(); // Просим Druid обновить интерфейс
    }
}
