use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Buttonkit<R>> {
  Ok(Buttonkit(app.clone()))
}

/// Access to the buttonkit APIs.
pub struct Buttonkit<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Buttonkit<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }

  /// Starts listening for physical button events
  pub fn start_listening(&self) -> crate::Result<()> {
    // No-op implementation for desktop
    // In a real implementation, you might register keyboard shortcuts or similar
    Ok(())
  }

  /// Stops listening for physical button events
  pub fn stop_listening(&self) -> crate::Result<()> {
    // No-op implementation for desktop
    Ok(())
  }
}
