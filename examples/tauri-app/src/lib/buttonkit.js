// Ejemplo de uso del plugin Buttonkit en una aplicación Tauri v2
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

export async function ping(value) {
  try {
    // Invocar el comando ping del plugin
    return await invoke('plugin:buttonkit|ping', { value });
  } catch (error) {
    console.error('Error executing ping:', error);
    throw error;
  }
}

export async function startListeningToButtons() {
  try {
    // Iniciar la escucha de botones físicos
    await invoke('plugin:buttonkit|start_listening_to_buttons');
    console.log('Escucha de botones físicos iniciada');
    return true;
  } catch (error) {
    console.error('Error al iniciar la escucha de botones:', error);
    return false;
  }
}

export async function stopListeningToButtons() {
  try {
    // Detener la escucha de botones físicos
    await invoke('plugin:buttonkit|stop_listening_to_buttons');
    console.log('Escucha de botones físicos detenida');
    return true;
  } catch (error) {
    console.error('Error al detener la escucha de botones:', error);
    return false;
  }
}

// Función para suscribirse a los eventos de botones físicos
export function onButtonPress(callback) {
  const unlistenPromise = listen('button-event', (event) => {
    const { button, timestamp } = event.payload;
    callback({ button, timestamp });
  });
  
  // Retorna una función para cancelar la suscripción
  return () => {
    unlistenPromise.then(unlisten => unlisten());
  };
}

// Ejemplo de uso:
/*
import { startListeningToButtons, stopListeningToButtons, onButtonPress } from './buttonkit';

// Iniciar la escucha de botones
await startListeningToButtons();

// Registrar un callback para los eventos de botones
const unsubscribe = onButtonPress((buttonEvent) => {
  console.log(`Botón presionado: ${buttonEvent.button}`);
  if (buttonEvent.button === 'VolumeUp') {
    // Hacer algo cuando se presiona el botón de subir volumen
  } else if (buttonEvent.button === 'VolumeDown') {
    // Hacer algo cuando se presiona el botón de bajar volumen
  }
});

// Cuando quieras dejar de escuchar los eventos:
unsubscribe();
stopListeningToButtons();
*/
