package com.plugin.buttonkit

import android.app.Activity
import android.content.Context
import android.view.KeyEvent
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import java.util.Date

@InvokeArg
class PingArgs {
  var value: String? = null
}

@TauriPlugin
class ButtonkitPlugin(private val activity: Activity): Plugin(activity) {
    private var isListening = false

    // Delegado de evento de teclado para manejar los botones de volumen
    private val keyEventListener = object : Activity.OnKeyListener {
        override fun onKey(v: android.view.View?, keyCode: Int, event: KeyEvent?): Boolean {
            if (event?.action == KeyEvent.ACTION_DOWN) {
                when (keyCode) {
                    KeyEvent.KEYCODE_VOLUME_UP -> {
                        sendButtonEvent("VolumeUp")
                        return true
                    }
                    KeyEvent.KEYCODE_VOLUME_DOWN -> {
                        sendButtonEvent("VolumeDown")
                        return true
                    }
                }
            }
            return false
        }
    }

    // Función para enviar un evento de botón al frontend
    private fun sendButtonEvent(buttonType: String) {
        val eventObject = JSObject()
        eventObject.put("button", buttonType)
        eventObject.put("timestamp", System.currentTimeMillis())
        
        // Enviar el evento a través del puente de Tauri
        activity.runOnUiThread {
            notifyListeners("button-event", eventObject)
        }
    }

    @Command
    fun ping(invoke: Invoke) {
        val args = invoke.parseArgs(PingArgs::class.java)

        val ret = JSObject()
        ret.put("value", args.value ?: "default value :(")
        invoke.resolve(ret)
    }

    @Command
    fun startListening(invoke: Invoke) {
        if (!isListening) {
            isListening = true
            
            // Registrar nuestra Activity como receptor de eventos de teclado
            activity.window.decorView.setOnKeyListener(keyEventListener)
            
            val result = JSObject()
            result.put("success", true)
            invoke.resolve(result)
        } else {
            invoke.resolve()
        }
    }

    @Command
    fun stopListening(invoke: Invoke) {
        if (isListening) {
            isListening = false
            
            // Eliminar nuestro receptor de eventos de teclado
            activity.window.decorView.setOnKeyListener(null)
            
            val result = JSObject()
            result.put("success", true)
            invoke.resolve(result)
        } else {
            invoke.resolve()
        }
    }
}
