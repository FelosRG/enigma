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
Para especificar la configuración inicial se puede usar una cadena de caracteres de por lo menos 3 caracteres de longitud. Al usar un string diferente se obtiene una codificación totalmente diferente del mismo texto.<br>

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

*💫 Protip 1: Una manera de evitar que el comando se guarde en el historial del shell es escribir un espacio antes del comando es decir usar "&nbsp; enigma 'texto...'" en vez de "enigma 'texto...'".*

### <div><b>Uso para generación de contraseña 🔒️</b></div>

*⚠️ No soy experto en seguridad y solo es una prueba de concepto.*

Supongamos que quieres generer alguna contraseña fuerte (mayusculas, minusculas, y caracteres especiales). Con enigma podemos generarnos una en base a otra contraseña mas facil de recordar que tengamos:

```
user@pc:$ enigma 'facebook' 'micontraseñafacil'
b3Tn@WKGtUñcAH4bQ
```

Incluso podemos genar más para distintos sitios.
```
user@pc:$ enigma 'twitter' 'micontraseñafacil'
ZD}9z7Wt~iñlr6{f]
```

*⚠️ Recomiendo personalizar las conexiones de los rotores si se usa el programa para este fin pues así nadie tendrá una maquina igual. Consultar la sección de guía de personalización.*

### <div><b> Instalación  ⚙️</b></div>

Se necesita tener el compilador de rust instalado.
```
cargo install --git https://github.com/FelosRG/enigma
```
Despues de la instalación se habilitará el comando *enigma*

### <div><b> Guía de Personalización </b></div>
to do..

