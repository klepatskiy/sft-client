use crate::state::data::AppState;
use druid::widget::{Widget, Button, Flex, Label, WidgetExt};

pub fn build_settings_page() -> impl Widget<AppState> {
    Flex::row()
        .with_flex_child(
            Flex::column()
                .with_child(Label::new("Settings").padding((10.0, 10.0)))
                .with_spacer(8.0)
                .with_child(
                    Button::new("Голос и видео")
                        .padding((10.0, 5.0))
                        .expand_width()
                        .on_click(|_ctx, _data: &mut AppState, _env| {
                            println!("Voice and Video settings clicked");
                        }),
                )
                .with_spacer(5.0)
                .with_child(
                    Button::new("Горящие клавиши")
                        .padding((10.0, 5.0))
                        .expand_width()
                        .on_click(|_ctx, _data: &mut AppState, _env| {
                            println!("Hotkeys settings clicked");
                        }),
                )
                .with_spacer(5.0)
                .with_child(
                    Button::new("Назад")
                        .padding((10.0, 5.0))
                        .expand_width()
                        .on_click(|_ctx, _data: &mut AppState, _env| {
                            println!("Back to setting");
                        }),
                )
                .background(druid::theme::BACKGROUND_DARK)
                .padding((10.0, 10.0)),
            1.0,
        )
        .with_flex_child(
            Label::new("Settings Content Area").center().padding(10.0),
            3.0,
        )
}
