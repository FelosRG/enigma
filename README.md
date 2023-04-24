<p align="center">
  <img src="fig/Enigma-logo.png" height=100>
</p>


## <div align="center"><b><a href="README.md">Español 🇲🇽</a> | <a href="README_ENG.md">English 🇺🇸</a></b></div>

La maquina enigma fue un dispositivo de cifrado de mensajes usado particularmente en la segunda guerra mundial por los alemanes. Fue considerada tan segura que era utilizada para cifrar los mensajes más sensibles durante la guerra. <br>


Este repositioro es un codificador de mensajes basado en el funcionamiento de las maquinas enigmas, algunas de las carácterísticas de esta implementación son:

📌 **Simulador de una maquina tipo enigma.** <br>
📌 **Más caracteres disponibles!** <br>
📌 **Estado inicial ajustable por medio de una cadena de texto.** <br>
📌 **Posible uso como codificador de contraseñas.** <br>
📌 **Simple. No librerías externas.** <br>
📌 **Facil uso.** <br>

### <div><b>Ejemplos </b></div>
Se puede usar esta implemnetación de dos formas distintas, usando la configuración por default o ingresando alguna configuración inicial dada como una cadena de caracteres que definirá la posición inicial de los rotores y los intercambio de letras.

#### <div><b> Usando la configuración por default</b></div>
Para cifrar solo hay que usar el comando *enigma* seguido del texto.

```
user@pc:$ enigma "Hola como estan todos!"
<5&< <xF* >fwb< a)+S5A
```
y para decifrar usamos *enigma* sobre el texto cifrado.
```
user@pc:$ enigma "<5&< <xF* >fwb< a)+S5A"
Hola como estan todos!
```
### <div><b> Instalación </b></div>

### <div><b> Guía de Personalización </b></div>

