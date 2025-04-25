use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_buttonkit);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<Buttonkit<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("com.plugin.buttonkit", "ButtonkitPlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_buttonkit)?;
  
  // Store a reference to the app handle for later use with events
  let buttonkit = Buttonkit { 
    handle,
    app: app.clone(),
  };
  
  Ok(buttonkit)
}

/// Access to the buttonkit APIs.
pub struct Buttonkit<R: Runtime> {
  handle: PluginHandle<R>,
  app: AppHandle<R>,
}

impl<R: Runtime> Buttonkit<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    self
      .handle
      .run_mobile_plugin("ping", payload)
      .map_err(Into::into)
  }
  
  /// Inicia la escucha de eventos de botones físicos
  pub fn start_listening(&self) -> crate::Result<()> {
    self
      .handle
      .run_mobile_plugin::<(), ()>("startListening", ())
      .map_err(Into::into)
  }
  
  /// Detiene la escucha de eventos de botones físicos
  pub fn stop_listening(&self) -> crate::Result<()> {
    self
      .handle
      .run_mobile_plugin::<(), ()>("stopListening", ())
      .map_err(Into::into)
  }
  
  /// Procesa un evento de botón recibido desde la plataforma móvil
  pub fn process_button_event(&self, event: ButtonEvent) {
    let _ = self.app.emit_all("button-event", event);
  }
}
