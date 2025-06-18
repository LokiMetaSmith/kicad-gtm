//ui.rs

use std::path::PathBuf;

use eframe::egui::{self, Color32, RichText};
use egui_modal::Modal;
// use log::debug;

use crate::Plugin;

pub trait Ui {
  fn draw_ui(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) -> Result<(), anyhow::Error>;
}

impl Ui for Plugin {
  fn draw_ui(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) -> Result<(), anyhow::Error> {
    let projects_folder = self.get_projects_folder();
    // api_key and api_url local variables removed
    let status = if !self.first_iteration_finished {
      "loading..."
    } else if projects_folder.as_os_str().is_empty() { // Condition updated
      "need settings!"
    } else {
      "OK"
    };
    let last_activity_label_text = match self.last_recorded_time_chrono { // Field name updated
      Some(dt) => dt.format("%H:%M:%S").to_string(),
      None => String::from("N/A"),
    };
    // settings window
    let modal = Modal::new(ctx, "settings");
    // luckily this call has a generic for the return type!
    modal.show(|ui| -> Result<(), anyhow::Error> {
      ui.label(RichText::new("kicad-gtm settings ^w^").size(16.0)); // Title updated
      ui.add_space(10.0);
      ui.label("track ALL projects in this folder:");
      // ui.text_edit_singleline(&mut self.watched_folder);
      ui.monospace(format!("{:?}", self.projects_folder)); // self.projects_folder is still a String here
      if ui.button("select folder").clicked() {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
          // self.projects_folder is a String, so we convert path to String.
          self.projects_folder = path.to_str().unwrap_or_default().to_string();
        }
      }
      // UI elements for API key and API URL removed
      if ui.button("OK").clicked() {
        self.set_projects_folder(self.projects_folder.clone());
        // Lines for set_api_key and set_api_url removed
        self.store_config()?;
        // Ensure self.projects_folder is correctly converted to PathBuf for watch_files
        let projects_folder_path = PathBuf::from(self.projects_folder.clone());
        if !projects_folder_path.as_os_str().is_empty() {
             self.watch_files(projects_folder_path)?;
        }
        modal.close();
      }
      Ok(())
    });
    // main window
    egui::CentralPanel::default().show(ctx, |ui| {
      // ui.heading("kicad-wakatime");
      ui.label(format!("status: {status}"));
      ui.label(format!("last activity recorded: {last_activity_label_text}")); // Label updated
      if ui.button("settings").clicked() {
        modal.open();
      }
      ui.add_space(20.0);
      ui.separator();
      egui_logger::logger_ui()
        .warn_color(Color32::YELLOW)
        .error_color(Color32::RED)
        .show(ui);
    });
    Ok(())
  }
}