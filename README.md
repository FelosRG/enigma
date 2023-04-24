<p align="center">
  <img src="fig/Enigma-logo.png" height=100>
</p>


## <div align="center"><b><a href="README.md">Español 🇲🇽</a> | <a href="README_ENG.md">English 🇺🇸</a></b></div>

La maquina enigma fue un dispositivo de cifrado de mensajes usado particularmente en la segunda guerra mundial por los alemanes. Fue considerada tan segura que era utilizada para cifrar los mensajes más sensibles durante la guerra. <br>


Este repositioro es un codificador de mensajes basado en el funcionamiento de las maquinas enigmas, algunas de las carácterísticas de esta implementación son:

📌 **Implementado en rust** <br>
📌 **Más caracteres disponibles!** <br>
📌 **Estado inicial ajustable por medio de una cadena de texto.** <br>
📌 **Posible uso como codificador de contraseñas.** <br>
📌 **Simple. No librerías externas.** <br>
📌 **Facil uso.** <br>

### <div><b>Ejemplos 📑</b></div>
Se puede usar esta implemnetación de dos formas distintas, usando la configuración por default o ingresando alguna configuración inicial dada como una cadena de caracteres que definirá la posición inicial de los rotores y los intercambio de letras.

#### <div><b> Usando la configuración por default</b></div>
Para cifrar solo hay que usar el comando *enigma* seguido del texto.

```
user@pc:$ enigma 'Hola como estan todos!'
<5&< <xF* >fwb< a)+S5A
```
y para decifrar usamos *enigma* sobre el texto cifrado.
```
user@pc:$ enigma '<5&< <xF* >fwb< a)+S5A'
Hola como estan todos!
```
#### <div><b> Con configuración inicial</b></div>
Para especificar la configuración inicial se puede usar una cadena de caracteres de por lo menos 3 caracteres de longitud. Puede ser cualquier caracter unicode válido.

```
user@pc:$ enigma 'cifrado1' 'Hola como estan todos!'
Vv(Z J@3d !Bf!g qwS*pR
user@pc:$ enigma 'cifrado1' 'Vv(Z J@3d !Bf!g qwS*pR'
Hola como estan todos!
```

```
user@pc:$ enigma '&%*@' 'Hola como estan todos!'
mgJ% Eg+) VId#$ WNpsr*
user@pc:$ enigma '&%*@' 'mgJ% Eg+) VId#$ WNpsr*'
Hola como estan todos!
```


### <div><b> Instalación  ⚙️</b></div>

Se necesita tener el compilador de rust instalado.
```
cargo install --git https://github.com/FelosRG/enigma
```
Despues de la instalación se habilitará el comando *enigma*

### <div><b> Guía de Personalización </b></div>

