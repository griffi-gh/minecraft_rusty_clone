use bevy::prelude::*;
use iyes_loopless::prelude::*;

use bevy_egui::{egui, EguiContext};
use crate::GameState;

#[derive(Default)]
struct MainMenuGuiState {}

fn main_menu_gui(
  mut commands: Commands,
  mut egui_context: ResMut<EguiContext>,
  //mut gui_state: ResMut<MainMenuGuiState>,
  mut exit: EventWriter<bevy::app::AppExit>
) {
  egui::Window::new("Main menu").show(egui_context.ctx_mut(), |ui| {
    if ui.button("Play").clicked() {
      commands.insert_resource(NextState(GameState::Connecting));
    }
    if ui.button("Exit").clicked() {
      exit.send_default();
    }
  });
}

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<MainMenuGuiState>();
    app.add_system(main_menu_gui.run_in_state(GameState::MainMenu));
  }
}
