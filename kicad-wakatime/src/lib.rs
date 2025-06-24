//lib.rs

use core::str;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read; // Cursor and Write removed
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use active_win_pos_rs::{get_active_window, ActiveWindow};
use chrono::{DateTime, Local};
use ini::Ini;
use log::debug;
use log::info;
use log::error;
use log::warn;
use notify::{Watcher, RecommendedWatcher, RecursiveMode};
use zip::ZipArchive;

pub mod ui;

const PLUGIN_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Plugin {
  pub version: &'static str,
  pub disable_gtm_recording: bool,
  pub kicad_wakatime_config: Ini,
  pub settings_open: bool,
  pub tx: Option<Sender<notify::Result<notify::Event>>>,
  pub rx: Option<Receiver<notify::Result<notify::Event>>>,
  // filename of currently focused file
  pub filename: String,
  // path of currently focused file
  pub full_path: PathBuf,
  pub full_paths: HashMap<String, PathBuf>,
  pub file_watcher: Option<RecommendedWatcher>,
  pub projects_folder: String,
  pub time: Duration,
  // the last time a heartbeat was recorded
  pub last_recorded_time: Duration,
  pub last_recorded_time_chrono: Option<DateTime<Local>>,
  // the last file that was recorded
  pub last_recorded_file: String,
  pub has_screen_capture_access: bool,
  pub first_iteration_finished: bool,
}

impl Plugin {
  pub fn new(disable_gtm_recording: bool) -> Self {
    Plugin {
      version: PLUGIN_VERSION,
      disable_gtm_recording,
      kicad_wakatime_config: Ini::default(),
      settings_open: false,
      tx: None,
      rx: None,
      filename: String::default(),
      full_path: PathBuf::default(),
      full_paths: HashMap::default(),
      file_watcher: None,
      projects_folder: String::default(),
      time: Duration::default(),
      last_recorded_time: Duration::default(),
      last_recorded_time_chrono: None,
      last_recorded_file: String::default(),
      has_screen_capture_access: true,
      first_iteration_finished: false,
    }
  }
  pub fn main_loop(&mut self) -> Result<(), anyhow::Error> {
    if !self.first_iteration_finished {
      let projects_folder = self.get_projects_folder();
      if !projects_folder.as_os_str().is_empty() {
        self.watch_files(projects_folder.clone())?;
      }
    }
    self.set_current_time(self.current_time());
    let Ok(w) = self.get_active_window() else {
      self.first_iteration_finished = true;
      return Ok(());
    };
    // TODO: maybe a regex would be way better for what we're about to do?
    // note: written this way, split can be Some for some things that aren't KiCAD, e.g. VS Code.
    // we sanity check it later.
    let split = w.title.split_once(" — ");
    let Some((mut project, editor)) = split else {
      self.first_iteration_finished = true;
      return Ok(());
    };

    // Check for KiCad placeholder titles
    if project == "[no schematic loaded]" || project == "[no pcb loaded]" {
        debug!("KiCad placeholder title detected: {}", project);
        self.first_iteration_finished = true;
        return Ok(());
    }

    // deal with unsaved files (asterisk prefix)
    if project.starts_with("*") {
      project = &project[1..project.len()];
    }

    debug!("Original project part from title: {}", project);
    // Deal with hierarchical schematics like "ProjectName [/SheetName]" or "ProjectName [SheetName]"
    // We want to extract "ProjectName"
    if let Some(bracket_pos) = project.rfind(" [") { // Look for " ["
        project = &project[0..bracket_pos];
    } else if let Some(bracket_pos) = project.find('[') { // If no " [" look for just "["
        // If '[' is at the start, this is not a typical hierarchical sheet path part we want to strip.
        // Or it could be a project name that starts with '[', e.g. "[OldProject]"
        // If bracket_pos is 0, slicing `&project[0..0]` is an empty string, which is not ideal.
        // Let's only strip if the bracket is not at the beginning.
        if bracket_pos > 0 {
            project = &project[0..bracket_pos];
        }
    }
    debug!("Processed project part for filename: {}", project);

    let filename = match editor {
      "Schematic Editor" => format!("{project}.kicad_sch"),
      "PCB Editor" => format!("{project}.kicad_pcb"),
      _ => String::new(), // If editor is unknown, filename will be empty
    };

    if filename.is_empty() {
        debug!("Unknown editor type or invalid project name for filename generation: editor='{}', project='{}'", editor, project);
        self.first_iteration_finished = true;
        return Ok(());
    }

    let Some(_full_path) = self.get_full_path(filename.clone()) else {
      debug!("Full path not found for filename: {}", filename);
      self.first_iteration_finished = true;
      return Ok(());
    };
    // let project_folder = full_path.parent().unwrap().to_path_buf();
    // let backups_folder = project_folder.join(format!("{project}-backups"));
    self.set_current_file(filename.clone())?;
    // self.look_at_backups_of_filename(filename, backups_folder);
    self.first_iteration_finished = true;
    Ok(())
  }
  pub fn get_active_window(&mut self) -> Result<ActiveWindow, ()> {
    let active_window = get_active_window();
    // as far as i can tell, active_win_pos_rs will focus on kicad-wakatime
    // when it starts, and that window should by all means have a title.
    // if the field is empty, kicad-wakatime is missing permissions
    if !self.has_screen_capture_access {
      if active_window.clone().is_ok_and(|w| w.app_name == "kicad-wakatime" && w.title.is_empty()) {
        error!("Could not get title of active window!");
        error!("If you are on macOS, please give kicad-wakatime Screen Recording permission");
        error!("(System Settings -> Privacy and Security -> Screen Recording)");
      }
    }
    active_window
  }
  pub fn load_config(&mut self) -> Result<(), anyhow::Error> {
    // kicad-wakatime config
    let kicad_wakatime_cfg_path = self.kicad_wakatime_cfg_path();
    if !fs::exists(&kicad_wakatime_cfg_path).unwrap() {
      Ini::new().write_to_file(&kicad_wakatime_cfg_path)?;
    }
    self.kicad_wakatime_config = Ini::load_from_file(&kicad_wakatime_cfg_path).unwrap();
    Ok(())
  }
  pub fn store_config(&self) -> Result<(), anyhow::Error> {
    Ini::write_to_file(&self.kicad_wakatime_config, self.kicad_wakatime_cfg_path())?;
    Ok(())
  }
  pub fn set_projects_folder(&mut self, projects_folder: String) {
    self.kicad_wakatime_config.with_section(Some("settings"))
      .set("projects_folder", projects_folder);
  }
  pub fn get_projects_folder(&mut self) -> PathBuf {
    match self.kicad_wakatime_config.with_section(Some("settings")).get("projects_folder") {
      Some(projects_folder) => PathBuf::from(projects_folder),
      None => PathBuf::new(),
    }
  }
  pub fn get_full_path(&self, filename: String) -> Option<&PathBuf> {
    self.full_paths.get(&filename)
  }
  pub fn recursively_add_full_paths(&mut self, path: PathBuf) -> Result<(), anyhow::Error> {
    for path_entry in fs::read_dir(path)? { // Renamed path to path_entry to avoid conflict
      let entry_path = path_entry.unwrap().path(); // Renamed path to entry_path
      if entry_path.is_dir() { self.recursively_add_full_paths(entry_path.clone())?; };
      if !entry_path.is_file() { continue; };
      let file_name = entry_path.file_name().unwrap().to_str().unwrap();
      let Some(file_extension) = entry_path.extension() else { continue; };
      let file_extension = file_extension.to_str().unwrap();
      if file_extension == "kicad_sch" || file_extension == "kicad_pcb" {
        if self.full_paths.contains_key(file_name) {
          error!("Found multiple files named {file_name} in the projects folder!");
          error!("Please select a folder that only contains one file named {file_name}!");
          self.full_paths = HashMap::new();
          return Ok(())
        }
        self.full_paths.insert(
          file_name.to_string(),
          entry_path // Use entry_path here
        );
      }
    }
    Ok(())
  }
  pub fn set_current_file(&mut self, filename: String) -> Result<(), anyhow::Error> {
    if self.filename != filename {
      info!("Focused file changed!");
      // since the focused file changed, it might be time to send a heartbeat.
      // self.filename and self.path are not actually updated here,
      // so self.maybe_record_gtm_activity() can use the difference as a condition in its check
      info!("Filename: {}", filename.clone());
      self.maybe_record_gtm_activity(filename.clone(), false)?;
      debug!("self.filename = {:?}", self.filename.clone());
      debug!("self.full_path = {:?}", self.full_path.clone());
    } else {
      // debug!("Focused file did not change!");
    }
    Ok(())
  }
  pub fn look_at_backups_of_filename(
    &mut self,
    filename: String,
    backups_folder: PathBuf
  ) -> Result<(), anyhow::Error> {
    // get all backups from the backups folder sorted by creation time
    info!("Looking at backups of {filename}...");
    std::thread::sleep(Duration::from_millis(500));
    let mut backups = fs::read_dir(backups_folder)?
      .flatten()
      .map(|x| x.path())
      .collect::<Vec<_>>();
    backups.sort_by_key(|x| x.metadata().unwrap().created().unwrap());
    let backups_count = backups.len();
    if backups_count < 2 { // Check to prevent panic
        info!("Not enough backups to compare for {filename}.");
        return Ok(());
    }
    let mut v1: Vec<u8> = vec![];
    let mut v2: Vec<u8> = vec![];
    let p1 = &backups[backups_count - 1];
    let p2 = &backups[backups_count - 2];
    let f1 = File::open(p1)?;
    let f2 = File::open(p2)?;
    let mut newest_backup = ZipArchive::new(f1)?;
    let mut second_newest_backup = ZipArchive::new(f2)?;
    let mut newest_backup_of_filename = newest_backup.by_name(&filename)?;
    let mut second_newest_backup_of_filename = second_newest_backup.by_name(&filename)?;
    newest_backup_of_filename.read_to_end(&mut v1)?;
    second_newest_backup_of_filename.read_to_end(&mut v2)?;
    if v1.ne(&v2) {
      info!("Change detected in backup!");
      self.maybe_record_gtm_activity(filename, false)?;
    } else {
      info!("No change detected in backup!");
    }
    Ok(())
  }
  pub fn create_file_watcher(&mut self) -> Result<(), anyhow::Error> {
    self.file_watcher = Some(notify::recommended_watcher(self.tx.clone().unwrap())?);
    Ok(())
  }
  pub fn watch_files(&mut self, path: PathBuf) -> Result<(), anyhow::Error> {
    if path == PathBuf::from("") {
      return Ok(())
    }
    info!("Watching {:?} for changes", path);
    self.create_file_watcher()?;
    self.file_watcher.as_mut().unwrap().watch(path.as_path(), RecursiveMode::Recursive)?;
    self.full_paths = HashMap::new();
    self.recursively_add_full_paths(path.clone())?;
    debug!("full_paths = {:?}", self.full_paths);
    Ok(())
  }
  pub fn try_recv(&mut self) -> Result<(), anyhow::Error> {
    let Some(ref rx) = self.rx else { unreachable!(); };
    let recv = rx.try_recv();
    if recv.is_ok() {
      if let Ok(Ok(notify::Event { kind, paths, attrs: _ })) = recv {
        let path = paths[0].clone();
        if path.parent().is_none() { return Ok(());} // Guard against panic
        let parent_str = path.parent().unwrap().to_str().unwrap_or_default(); // Avoid panic
        let is_backup = parent_str.ends_with("-backups");

        if path == self.full_path {
          info!("File saved!");
          self.maybe_record_gtm_activity(self.filename.clone(), true)?;
        } else if is_backup && kind.is_create() {
          info!("New backup created!");
          if let Some(parent_path) = path.parent() { // Ensure parent path exists
             self.look_at_backups_of_filename(self.filename.clone(), parent_path.to_path_buf())?;
          }
        }
      }
    }
    Ok(())
  }
  pub fn current_time(&self) -> Duration {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards!")
  }
  pub fn set_current_time(&mut self, t: Duration) {
    self.time = t;
  }
  /// Return the amount of time passed since the last heartbeat.
  pub fn time_passed(&self) -> Duration {
    self.current_time() - self.last_recorded_time
  }
  /// Returns `true` if more than 2 minutes have passed since the last GTM activity.
  pub fn enough_time_passed(&self) -> bool {
    self.time_passed() > Duration::from_secs(120)
  }
  /// Record GTM activity if conditions are met.
  pub fn maybe_record_gtm_activity(
    &mut self,
    filename: String,
    is_file_saved: bool
  ) -> Result<(), anyhow::Error> {
    debug!("Determining whether to record GTM activity...");
    if self.last_recorded_time == Duration::ZERO {
      debug!("No GTM activity has been recorded since the plugin opened");
    } else {
      debug!("It has been {:?} since the last GTM activity", self.time_passed());
    }
    if self.time_passed() < Duration::from_millis(1000) { // Prevent too frequent recordings
      debug!("Not recording GTM activity (too fast!)");
      return Ok(())
    }
    if is_file_saved ||
    self.enough_time_passed() ||
    self.filename != filename {
      self.filename = filename.clone();
      match self.get_full_path(filename.clone()) {
          Some(path_buf) => self.full_path = path_buf.to_path_buf(),
          None => {
              error!("Could not find full path for filename: {}", filename);
              return Ok(());
          }
      }
      self.record_gtm_activity()?;
    } else {
      debug!("Not recording GTM activity (no conditions met)");
    }
    Ok(())
  }

  pub fn record_gtm_activity(&mut self) -> Result<(), anyhow::Error> {
    info!("Recording GTM activity...");
    if self.disable_gtm_recording {
      warn!("GTM recording is disabled (using --disable-gtm-recording)");
      warn!("Updating last_recorded_time anyway");
      self.last_recorded_time = self.current_time();
      self.last_recorded_time_chrono = Some(Local::now());
      return Ok(())
    }

    let full_path_string = self.full_path.clone().into_os_string().into_string()
        .map_err(|os_string| anyhow::anyhow!("Failed to convert path to string: {:?}", os_string))?;

    // Log the exact command string that will be attempted.
    info!("Executing GTM CLI: gtm record \"{}\"", full_path_string);

    let mut cmd = std::process::Command::new("gtm");
    cmd.arg("record");
    cmd.arg(&full_path_string);

    let cli_output = cmd.output();

    match cli_output {
        Ok(output) => {
            debug!("gtm record status = {}", output.status);
            let stdout = str::from_utf8(&output.stdout).unwrap_or_default();
            let stderr = str::from_utf8(&output.stderr).unwrap_or_default();
            debug!("gtm record stdout = {:?}", stdout);
            debug!("gtm record stderr = {:?}", stderr);
            if !output.status.success() {
                error!("gtm record command failed with status: {}", output.status);
                error!("gtm stderr: {}", stderr);
            }
        }
        Err(e) => {
            error!("Failed to execute gtm record command: {}", e);
            if e.kind() == std::io::ErrorKind::NotFound {
                error!("'gtm' command not found. Please ensure GTM is installed and in your system's PATH.");
            }
            return Err(e.into());
        }
    }

    info!("GTM activity recording finished!");
    self.last_recorded_time = self.current_time();
    self.last_recorded_time_chrono = Some(Local::now());
    self.last_recorded_file = full_path_string;
    debug!("last_recorded_time = {:?}", self.last_recorded_time);
    debug!("last_recorded_file = {:?}", self.last_recorded_file);
    Ok(())
  }

  /// Return the path to the .kicad-wakatime.cfg file.
  pub fn kicad_wakatime_cfg_path(&self) -> PathBuf {
    let home_dir = home::home_dir().expect("Unable to get your home directory!");
    home_dir.join(".kicad-wakatime.cfg") // This will likely be renamed to .kicad-gtm.cfg later
  }
}