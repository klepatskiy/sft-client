use druid::{Target, Widget};
use druid::widget::{Button, Flex};
use crate::delegate::SWITCH_TO_SETTINGS;
use crate::state::data::AppViewState;
use crate::state::header::HeaderState;

pub fn build_header_ui() -> impl Widget<HeaderState> {
    Flex::row()
        .with_child(Button::new("🎤").on_click(|_ctx, _data: &mut HeaderState, _env| {
            println!("Microphone button clicked");
        }))
        .with_child(Button::new("🔊").on_click(|_ctx, _data: &mut HeaderState, _env| {
            println!("Speaker button clicked");
        }))
        .with_child(Button::new("⚙️").on_click(|ctx, _data: &mut HeaderState, _env| {
            ctx.submit_command(druid::Command::new(SWITCH_TO_SETTINGS, AppViewState::Setting, Target::Global));
        }))
        .with_spacer(10.0)
}