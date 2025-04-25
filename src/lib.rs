use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Buttonkit;
#[cfg(mobile)]
use mobile::Buttonkit;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the buttonkit APIs.
pub trait ButtonkitExt<R: Runtime> {
  fn buttonkit(&self) -> &Buttonkit<R>;
}

impl<R: Runtime, T: Manager<R>> crate::ButtonkitExt<R> for T {
  fn buttonkit(&self) -> &Buttonkit<R> {
    self.state::<Buttonkit<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("buttonkit")
    .invoke_handler(tauri::generate_handler![
      commands::ping,
      commands::start_listening_to_buttons,
      commands::stop_listening_to_buttons
    ])
    .setup(|app, api| {
      #[cfg(mobile)]
      let buttonkit = mobile::init(app, api)?;
      #[cfg(desktop)]
      let buttonkit = desktop::init(app, api)?;
      app.manage(buttonkit);
      Ok(())
    })
    .build()
}
