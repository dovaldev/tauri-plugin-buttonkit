use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::ButtonkitExt;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.buttonkit().ping(payload)
}

#[command]
pub(crate) async fn start_listening_to_buttons<R: Runtime>(
    app: AppHandle<R>,
) -> Result<()> {
    app.buttonkit().start_listening()
}

#[command]
pub(crate) async fn stop_listening_to_buttons<R: Runtime>(
    app: AppHandle<R>,
) -> Result<()> {
    app.buttonkit().stop_listening()
}
