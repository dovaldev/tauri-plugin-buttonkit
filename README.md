# Tauri Plugin Buttonkit

Un plugin para aplicaciones Tauri que permite detectar pulsaciones de botones físicos en dispositivos móviles, como los botones de volumen en Android.

## Compatibilidad

- ✅ Android
- ✅ iOS (pendiente de implementar completamente)

## Instalación

### 1. Añadir la dependencia Rust a tu aplicación Tauri

En tu archivo `src-tauri/Cargo.toml`, añade:

```toml
[dependencies]
tauri-plugin-buttonkit = "0.1.0"
```

O si prefieres usar la versión en desarrollo directamente desde GitHub:

```toml
[dependencies]
tauri-plugin-buttonkit = { git = "https://github.com/dovaldev/tauri-plugin-buttonkit" }
```

### 2. Inicializar el plugin en tu aplicación

En tu archivo `src-tauri/src/main.rs`, añade el plugin al builder de Tauri:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_buttonkit::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 3. Instalar el paquete JavaScript/TypeScript

```bash
# Usando npm
npm install tauri-plugin-buttonkit-api

# Usando yarn
yarn add tauri-plugin-buttonkit-api

# Usando pnpm
pnpm add tauri-plugin-buttonkit-api
```

## Uso

### JavaScript/TypeScript

```javascript
import { 
  ping, 
  startListeningToButtons, 
  stopListeningToButtons, 
  onButtonPress 
} from 'tauri-plugin-buttonkit-api';

// Ejemplo básico para verificar que el plugin funciona
ping("Hola desde la app!").then(response => {
  console.log(response); // Debería mostrar el valor que enviaste
});

// Iniciar la escucha de botones
await startListeningToButtons();

// Registrar un callback para los eventos de botones
const unsubscribe = onButtonPress((event) => {
  console.log(`Botón presionado: ${event.button}`);
  
  // Realizar acciones según el botón presionado
  if (event.button === 'VolumeUp') {
    // Hacer algo cuando se presiona el botón de subir volumen
  } else if (event.button === 'VolumeDown') {
    // Hacer algo cuando se presiona el botón de bajar volumen
  }
});

// Cuando quieras dejar de escuchar los eventos:
unsubscribe();
await stopListeningToButtons();
```

### Ejemplo con React y TypeScript

```tsx
import React, { useState, useEffect } from 'react';
import { 
  ping, 
  onButtonPress, 
  startListeningToButtons, 
  stopListeningToButtons,
  ButtonEvent 
} from 'tauri-plugin-buttonkit-api';

// Definimos el tipo para los eventos de botones procesados
interface ProcessedButtonEvent {
  button: string;
  time: string;
}

const ButtonKitComponent: React.FC = () => {
  const [buttonEvents, setButtonEvents] = useState<ProcessedButtonEvent[]>([]);
  const [isListening, setIsListening] = useState<boolean>(false);
  const [unsubscribeFunc, setUnsubscribeFunc] = useState<(() => Promise<void>) | null>(null);

  // Función para manejar los eventos de botones
  const handleButtonPress = (event: ButtonEvent) => {
    const { button, timestamp } = event;
    const newEvent = {
      button,
      time: new Date(timestamp).toLocaleTimeString()
    };
    
    setButtonEvents(prevEvents => {
      // Mantener solo los últimos 10 eventos
      const updatedEvents = [...prevEvents, newEvent];
      if (updatedEvents.length > 10) {
        return updatedEvents.slice(updatedEvents.length - 10);
      }
      return updatedEvents;
    });
  };

  // Función para iniciar la escucha de botones
  const startListening = async () => {
    if (!isListening) {
      try {
        await startListeningToButtons();
        const unsubscribe = onButtonPress(handleButtonPress);
        setUnsubscribeFunc(() => unsubscribe);
        setIsListening(true);
        console.log("Escucha de botones iniciada");
      } catch (error) {
        console.error("Error al iniciar la escucha:", error);
      }
    }
  };

  // Función para detener la escucha de botones
  const stopListening = async () => {
    if (isListening && unsubscribeFunc) {
      try {
        await unsubscribeFunc();
        await stopListeningToButtons();
        setIsListening(false);
        setUnsubscribeFunc(null);
        console.log("Escucha de botones detenida");
      } catch (error) {
        console.error("Error al detener la escucha:", error);
      }
    }
  };

  // Efecto para iniciar automáticamente en dispositivos móviles
  useEffect(() => {
    const isMobileDevice = /Android|iPhone|iPad|iPod/i.test(navigator.userAgent);
    if (isMobileDevice) {
      startListening();
    }

    // Limpiar al desmontar el componente
    return () => {
      if (isListening && unsubscribeFunc) {
        unsubscribeFunc().then(() => {
          stopListeningToButtons().catch(console.error);
        }).catch(console.error);
      }
    };
  }, []);

  return (
    <div>
      <h2>Control de botones físicos</h2>
      
      <div>
        <button 
          onClick={startListening} 
          disabled={isListening}
        >
          Iniciar escucha
        </button>
        <button 
          onClick={stopListening} 
          disabled={!isListening}
        >
          Detener escucha
        </button>
      </div>

      <div>
        <h3>Eventos registrados:</h3>
        {buttonEvents.length === 0 ? (
          <p>No se han detectado eventos de botones físicos aún.</p>
        ) : (
          <ul>
            {buttonEvents.map((event, index) => (
              <li key={index}>{event.time} - Botón: {event.button}</li>
            ))}
          </ul>
        )}
      </div>
    </div>
  );
};

export default ButtonKitComponent;
```

## API

### `ping(value: string): Promise<PingResponse>`

Función de prueba que devuelve el valor enviado. Útil para verificar que el plugin está funcionando correctamente.

### `startListeningToButtons(): Promise<void>`

Inicia la escucha de eventos de botones físicos en el dispositivo.

### `stopListeningToButtons(): Promise<void>`

Detiene la escucha de eventos de botones físicos.

### `onButtonPress(callback: (event: ButtonEvent) => void): () => Promise<void>`

Registra una función callback que será llamada cada vez que se presione un botón físico. Retorna una función para cancelar la suscripción.

#### Tipos

```typescript
interface ButtonEvent {
  button: string;  // Identificador del botón (ej: "VolumeUp", "VolumeDown")
  timestamp: number;  // Timestamp en milisegundos cuando ocurrió el evento
}

interface PingResponse {
  value: string;  // El valor enviado en la función ping
}
```

## Botones soportados

Actualmente, el plugin detecta los siguientes botones en Android:
- `VolumeUp`: Botón de subir volumen
- `VolumeDown`: Botón de bajar volumen

## Desarrollo

### Prerequisitos
- Rust y Cargo
- Node.js y npm/yarn/pnpm
- Tauri CLI
- Android Studio (para desarrollo en Android)
- Xcode (para desarrollo en iOS)

### Estructura del proyecto
- `src/`: Código Rust del plugin
- `guest-js/`: Código TypeScript de la API del frontend
- `android/`: Implementación nativa para Android
- `ios/`: Implementación nativa para iOS
- `examples/`: Aplicación de ejemplo que usa el plugin

### Ejecutar el ejemplo
```bash
cd examples/tauri-app
pnpm install
pnpm tauri android dev
```

## Licencia

Este proyecto está bajo la [Licencia MIT](LICENSE).

## Contribuciones

Las contribuciones son bienvenidas. Por favor, abre un issue o envía un pull request en GitHub.
