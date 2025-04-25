import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export interface ButtonEvent {
  button: string;
  timestamp: number;
}

export interface PingResponse {
  value: string;
}

export async function ping(value: string): Promise<PingResponse> {
  return await invoke<PingResponse>('plugin:buttonkit|ping', {
    payload: {
      value,
    },
  });
}

export async function startListeningToButtons(): Promise<void> {
  await invoke('plugin:buttonkit|start_listening_to_buttons');
}

export async function stopListeningToButtons(): Promise<void> {
  await invoke('plugin:buttonkit|stop_listening_to_buttons');
}

export function onButtonPress(callback: (event: ButtonEvent) => void): () => Promise<void> {
  const unlistenPromise = listen<ButtonEvent>('button-event', (event) => {
    callback(event.payload);
  });
  
  return async () => {
    const unlisten = await unlistenPromise;
    unlisten();
  };
}
