use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use time::{ self, OffsetDateTime };
use shared::types::ChatMessage;

#[derive(Default)]
pub struct ChatMessages(pub Vec<ChatMessage>);

fn chat_gui(
  mut egui_context: ResMut<EguiContext>,
  messages: Res<ChatMessages>,
) {
  egui::Window::new("Chat").show(egui_context.ctx_mut(), |ui| {
    //TODO scrolling and ability to send messages
    for message in &messages.0 {
      ui.horizontal(|ui| {
        let timestamp: OffsetDateTime = message.timestamp.into();
        let format = time::format_description::parse("[hour]:[minute]").unwrap();
        ui.label(format!("[{}] {} : ", timestamp.format(&format).unwrap(), &message.from));
        ui.label(&message.message);
      });
    }
  });
}

pub struct ChatPlugin;
impl Plugin for ChatPlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<ChatMessages>();
    app.add_system(chat_gui);
  }
}
