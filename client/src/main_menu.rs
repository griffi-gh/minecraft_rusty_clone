use bevy::prelude::*;
use iyes_loopless::prelude::*;

use bevy_egui::{egui::{self, Align2, Vec2 as EVec2}, EguiContext};
use crate::GameState;

#[derive(Default, PartialEq)]
#[non_exhaustive]
enum MainMenuScreen {
  #[default]
  Main,
  Connect
}

#[derive(Default)]
struct MainMenuGuiState {
  screen: MainMenuScreen
}

fn main_menu_gui(
  mut commands: Commands,
  mut egui_context: ResMut<EguiContext>,
  mut gui_state: ResMut<MainMenuGuiState>,
  mut exit: EventWriter<bevy::app::AppExit>
) {
  egui::Window::new("Main menu")
    .collapsible(false)
    .default_width(300.)
    .anchor(Align2::CENTER_CENTER, EVec2::ZERO)
    .show(egui_context.ctx_mut(), |ui| {
      ui.vertical_centered_justified(|ui| {

        //Stuff
        match gui_state.screen {
          MainMenuScreen::Main => {
            if ui.button("Play").clicked() {
              gui_state.screen = MainMenuScreen::Connect;
            }
            if ui.button("Exit").clicked() {
              exit.send_default();
            }
          },
          MainMenuScreen::Connect => {
            if ui.button("[DEBUG]\nConnect to localhost").clicked() {
              commands.insert_resource(NextState(GameState::Connecting));
            }
          }
        }

        //Back button
        if gui_state.screen != MainMenuScreen::Main {
          if ui.button("<= Back").clicked() {
            gui_state.screen = MainMenuScreen::Main;
          }
        }

      });
    });
}

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
  fn build(&self, app: &mut App) {
    app.init_resource::<MainMenuGuiState>();
    app.add_system(main_menu_gui.run_in_state(GameState::MainMenu));
  }
}
