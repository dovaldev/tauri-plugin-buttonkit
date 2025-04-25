<script>
  import { onDestroy, onMount } from 'svelte';
  import { onButtonPress, ping, startListeningToButtons, stopListeningToButtons } from './lib/buttonkit.js';
  import Greet from './lib/Greet.svelte';

  let response = ''
  let buttonEvents = []
  let unsubscribe = null
  let isListening = false

  function updateResponse(returnValue) {
    response += `[${new Date().toLocaleTimeString()}] ` + (typeof returnValue === 'string' ? returnValue : JSON.stringify(returnValue)) + '<br>'
  }

  function _ping() {
    ping("Pong!").then(updateResponse).catch(updateResponse)
  }

  // Función para manejar los eventos de botones
  function handleButtonPress(event) {
    const { button, timestamp } = event;
    const time = new Date(timestamp).toLocaleTimeString();
    buttonEvents = [...buttonEvents, { button, time }];
    
    // Mantener solo los últimos 10 eventos
    if (buttonEvents.length > 10) {
      buttonEvents = buttonEvents.slice(buttonEvents.length - 10);
    }
  }

  // Función para iniciar la escucha de botones
  async function startListening() {
    if (!isListening) {
      await startListeningToButtons();
      unsubscribe = onButtonPress(handleButtonPress);
      isListening = true;
      updateResponse("Escucha de botones iniciada");
    }
  }

  // Función para detener la escucha de botones
  async function stopListening() {
    if (isListening && unsubscribe) {
      unsubscribe();
      await stopListeningToButtons();
      isListening = false;
      updateResponse("Escucha de botones detenida");
    }
  }

  // Iniciar la escucha cuando se monta el componente
  onMount(() => {
    // En dispositivos móviles, activamos automáticamente la escucha
    if (navigator.userAgent.match(/Android/i) || navigator.userAgent.match(/iPhone|iPad|iPod/i)) {
      startListening();
    }
  });

  // Detener la escucha cuando se destruye el componente
  onDestroy(() => {
    if (isListening) {
      stopListening();
    }
  });
</script>

<main class="container">
  <h1>Welcome to Tauri Buttonkit!</h1>

  <div class="row">
    <a href="https://vite.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>

  <p>
    Este ejemplo muestra cómo detectar la pulsación de botones físicos de volumen en dispositivos Android.
  </p>

  <div class="row">
    <Greet />
  </div>

  <div class="button-section">
    <button on:click="{_ping}">Ping</button>
    <button on:click="{startListening}" disabled="{isListening}">Iniciar escucha de botones</button>
    <button on:click="{stopListening}" disabled="{!isListening}">Detener escucha de botones</button>
  </div>

  <div class="button-events">
    <h2>Eventos de botones físicos</h2>
    {#if buttonEvents.length === 0}
      <p>No se han detectado eventos de botones. Intenta pulsar los botones de volumen de tu dispositivo.</p>
    {:else}
      <ul>
        {#each buttonEvents as event}
          <li>{event.time} - Botón: {event.button}</li>
        {/each}
      </ul>
    {/if}
  </div>

  <div>
    <h3>Respuestas</h3>
    <div>{@html response}</div>
  </div>

</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }

  .logo.tauri:hover {
    filter: drop-shadow(0 0 2em #24c8db);
  }

  .button-section {
    display: flex;
    gap: 10px;
    margin: 20px 0;
  }

  .button-events {
    margin: 20px 0;
    padding: 15px;
    border: 1px solid #ccc;
    border-radius: 8px;
    background-color: #f8f8f8;
  }

  button {
    background-color: #24c8db;
    border: none;
    color: white;
    padding: 10px 15px;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.3s;
  }

  button:hover {
    background-color: #1fa3b3;
  }

  button:disabled {
    background-color: #cccccc;
    cursor: not-allowed;
  }

  ul {
    list-style-type: none;
    padding: 0;
  }

  li {
    padding: 8px;
    border-bottom: 1px solid #eee;
  }

  li:last-child {
    border-bottom: none;
  }
</style>
