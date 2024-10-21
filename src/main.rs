use druid::{AppLauncher, WindowDesc};
use page::auth::build_auth_ui;
use state::AuthState;

mod page;
mod state;
mod client;

fn main() {
    // Создаем главное окно
    let main_window = WindowDesc::new(build_auth_ui())
        .title("Login Form")
        .window_size((300.0, 200.0));

    // Начальное состояние приложения
    let initial_state = AuthState {
        email: "".into(),
        login: "".into(),
        display_message: "Please enter your credentials".into(),
        is_logged_in: false, // Пользователь не залогинен по умолчанию
    };

    // Запуск приложения
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
