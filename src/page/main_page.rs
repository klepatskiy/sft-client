use druid::{Widget, WidgetExt};
use druid::widget::{Flex, Label};
use crate::page::header::build_header_ui;
use crate::state::data::AppState;

pub fn build_main_ui() -> impl Widget<AppState> {
    Flex::column()
        .with_child(build_header_ui().lens(AppState::settings)) // Добавляем header сверху
        .with_flex_child(
            Flex::row()
                // .with_flex_child(
                //     build_friend_list().lens(AppState::friends), // Список друзей слева
                //     1.0,
                // )
                .with_flex_child(
                    Label::new("Welcome to the main interface") // Основной контент по центру
                        .center()
                        .padding(10.0),
                    3.0,
                ),
            1.0,
        )
}

// pub fn build_friend_list() -> impl Widget<FriendState> {
//     List::new(|friend: &Friend, _| {
//         Label::new(|data: &Friend, _env: &_| format!("Friend: {}", data.name))
//     })
//         .lens(FriendState::friends)
//         .padding(5.0)
// }
