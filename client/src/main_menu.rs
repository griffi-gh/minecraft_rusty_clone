use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy_egui::{
  egui::{self, Align2, Vec2 as EVec2, Color32}, 
  EguiContext
};
use reqwest;
use serde_json::Value as JsonValue;
use std::net::{SocketAddr, IpAddr};
use shared::utils::{check_username, check_password};
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
  screen: MainMenuScreen,
  server_addr: String,
  username: String,
  password: Option<String>,
}

fn main_menu_gui(
  mut commands: Commands,
  mut egui_context: ResMut<EguiContext>,
  mut gui_state: ResMut<MainMenuGuiState>,
  mut exit: EventWriter<bevy::app::AppExit>
) {
  egui::Window::new("Main menu")
    .collapsible(false)
    .resizable(false)
    .title_bar(false)
    .default_width(300.)
    .anchor(Align2::CENTER_CENTER, EVec2::ZERO)
    .show(egui_context.ctx_mut(), |ui| {
      ui.vertical_centered_justified(|ui| {

        //Stuff
        match gui_state.screen {
          MainMenuScreen::Main => {
            if ui.button("\nPlay\n").clicked() {
              gui_state.screen = MainMenuScreen::Connect;
            }
            if ui.button("\nExit\n").clicked() {
              exit.send_default();
            }
          },
          MainMenuScreen::Connect => {

            let mut form_valid = true;

            { //USERNAME INPUT BOX
              let valid = check_username(gui_state.username.as_str());
              form_valid &= valid.is_ok();
              ui.add(
                egui::TextEdit::singleline(&mut gui_state.username)
                  .text_color(if valid.is_ok() { Color32::LIGHT_GREEN } else { Color32::LIGHT_RED })
                  .hint_text("Username")
              );
            }

            ui.add_enabled_ui(gui_state.password.is_none(), |ui| {
              //SERVER IP INPUT BOX
              let is_valid = 
                gui_state.server_addr.parse::<SocketAddr>().is_ok() ||
                gui_state.server_addr.parse::<IpAddr>().is_ok();
              form_valid &= is_valid;
              ui.add(
                egui::TextEdit::singleline(&mut gui_state.server_addr)
                  .text_color(if is_valid { Color32::LIGHT_GREEN } else { Color32::LIGHT_RED })
                  .hint_text("Server address")
              );
            });
            

            //PASSWORD INPUT BOX
            if let Some(password) = gui_state.password.as_mut() {
              ui.separator();
              ui.label("This server is password-protected");
              let pwd_valid = check_password(password.as_str()).is_ok();
              form_valid &= pwd_valid;
              ui.add(
                egui::TextEdit::singleline(password)
                  .password(true)
                  .text_color(if pwd_valid { Color32::LIGHT_GREEN } else { Color32::LIGHT_RED })
                  .hint_text("Enter the server password")
              );
              ui.separator();
            }

            ui.add_enabled_ui(form_valid, |ui| {
              if ui.button("Connect").clicked() {
                let mut proceed = true;
                if gui_state.password.is_none() {
                  if let Ok(res) = reqwest::blocking::get(format!("http://{}/", gui_state.server_addr)) {
                    if let Ok(json_val) = res.json::<JsonValue>() {
                      let has_pwd = json_val["password_protected"].as_bool().unwrap_or_default();
                      if has_pwd {
                        gui_state.password = Some(String::new());
                        proceed = false;
                      }
                    } else { proceed = false }
                  } else { proceed = false }
                }
                if proceed {
                  commands.insert_resource(NextState(GameState::Connecting));
                }
              }
            });

            if ui.button("[DEBUG] Connect to localhost").clicked() {
              commands.insert_resource(NextState(GameState::Connecting));
            }
          }
        }

        //Back button
        if gui_state.screen != MainMenuScreen::Main {
          ui.separator();
          if ui.button("<= Back").clicked() {
            *gui_state = default();
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
