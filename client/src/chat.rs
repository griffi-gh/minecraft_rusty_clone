use bevy::prelude::*;
use iyes_loopless::prelude::*;

use bevy_egui::{
  egui::{self, RichText, Color32},
  EguiContext
};
use std::time::SystemTime;
use time::{ self, OffsetDateTime };
use shared::types::ChatMessage;
use crate::{
  GameState,
  networking::RequestNetChatSend
};

pub struct ChatMessageEvent(String);

#[derive(Default)]
pub struct ChatMessages(pub Vec<ChatMessage>);

#[derive(Default)]
struct ChatGuiState {
  pub chat_message: String,
}


fn chat_gui(
  mut egui_context: ResMut<EguiContext>,
  mut gui_state: ResMut<ChatGuiState>,
  mut msg_event: EventWriter<ChatMessageEvent>,
  messages: Res<ChatMessages>,
) {
  egui::Window::new("Chat").show(egui_context.ctx_mut(), |ui| {
    //TODO scrolling and ability to send messages
    for message in &messages.0 {
      ui.horizontal(|ui| {
        let time_text = {
          let timestamp: OffsetDateTime = message.timestamp.into();
          let format = time::format_description::parse("[hour]:[minute]").unwrap();
          format!("[{}] {} : ", timestamp.format(&format).unwrap(), &message.from)
        };
        ui.label(RichText::new(time_text).color(Color32::GRAY));
        ui.label(&message.message);
      });
    }
    ui.horizontal(|ui| {
      ui.add(egui::TextEdit::singleline(&mut gui_state.chat_message).hint_text("Type your chat message"));
      if ui.button("Send").clicked() {
        msg_event.send(
          ChatMessageEvent(gui_state.chat_message.clone())
        );
        gui_state.chat_message = String::default();
      }
    });
  });
}

fn chat_send(
  mut msg_event: EventReader<ChatMessageEvent>,
  mut msg_net_event: EventWriter<RequestNetChatSend>,
  mut messages: ResMut<ChatMessages>,
) {
  for evt in msg_event.iter() {
    msg_net_event.send(RequestNetChatSend(evt.0.clone()));
    messages.0.push(ChatMessage {
      message: evt.0.clone(),
      from: "[YOU]".into(), //TODO insert real username
      timestamp: SystemTime::now(),
      is_system: false
    });
  }
}

fn reset_chat (
  mut messages: ResMut<ChatMessages>,
  mut gui_state: ResMut<ChatGuiState>,
) {
  messages.0 = Vec::new();
  *gui_state = default();
}

pub struct ChatPlugin;
impl Plugin for ChatPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<ChatMessageEvent>();
    app.init_resource::<ChatGuiState>();
    app.init_resource::<ChatMessages>();
    app.add_system(chat_gui.chain(chat_send).run_in_state(GameState::InGame));
    app.add_exit_system(GameState::InGame, reset_chat);
  }
}
