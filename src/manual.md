# Sintaxis

#### Variables

```
var a = 20;
var b = verdad == falso;
var c = [0, 1, "hola", {"juan": 20}];
a = b;
```

#### Condicionales

```
var a = 20;
si a == 20 {
    var b = a * 2;
}
sino {
    var c = verdad != falso;
}
```

#### Funciones

```
var sumar = fn(x, y) {
    var extra = 2;
    x + y + extra;
}

fn resta(x, y) {
    x - y;
}

var total = sumar(1, 2) * resta(4, 5);
```

#### Comentarios

```
#Hola esto es un comentario.
#Lo mismo pero en otra linea.
var a = ""; #Ahora digo que hace esta variable, nada.
```

#### While loops

```
mientras verdad {
    imprimir("Hola mundo");
}
```

#### for loops

```
# Fibonacci
var a = 0;
var b = 1;
para i en rango(30) {
    var c = a + b;
    a = b;
    b = c;
    imprimir(b);
}
```

# Tipo de datos

```
# Numerico
var a = 15 + 0b1111 # -> 30;
a = 0xffff # -> 255
a = 0o10 # -> 8
a = 0.2231 # -> Lo mismo

# Logico
var b = verdad;

# Cadena
var c = "Hola mundo";

# Nulo
var d = nulo;

# Lista
var e = [1, 2 "hola", [fn(x) { x * 2}, d]];
e[0] = 10

# Diccionario
var g = {verdad: [{1: 10}, "xd", falso], "hola": "mundo"}
g["hola"] = nulo;
```

# Referencia y copias

Los tipos de datos: `Numericos`, `logicos`, `nulo` y `cadena`. No pueden ser referenciados, se copian con cada asignacion.

```
var a = 10;
var b = a; # b copia el valor de a
```

Los tipos de datos: `Lista` y `Diccionario` pueden son referenciados.

```
var a = [0, 1, 2]
var b = a; # b referencia a
b[1] = "hola mundo"
imprime(a[1]) # -> "hola mundo"

```

# Operaciones

```
falso == 0                  #-> verdad
falso != verdad             #-> verdad
"hola" == "chao"            #-> falso
4 < 0                       #-> falso
9 > 8                       #-> verdad
1 >= 1                      #-> verdad
0 <= 1                      #-> verdad
nulo != 2                   #-> verdad
[1, 2, [3, 4]] != [1, 2]    #-> verdad
[1, 2, 3] > [1, 2]          #-> verdad
1 + 2                       #-> 3
2 - 4                       #-> -2
4 * 4                       #-> 16
2 / 2                       #-> 0
verdad - 1                  #-> 0
"hola" + " " + "mundo"      #-> "hola mundo"
[1, 2] + [3, 4]             #-> [1, 2, 3, 4]
2 == [0, 2][1]              #->
"hola" * 2                  #-> "holahola"
[1, 2, [3, 4]] * 2          #-> [1, 2, [3, 4], 1, 2, [3, 4]]
a += 1
b -= 1
c *= 1
d /= 1
e *= 1
```

# Funciones internas

##### Longitud

```
longitud("hola")                                    # -> 4
longitud([1, 2, 3])                                 # -> 3
longitud({"nombre": "Sebas", "apellido": "Lopez"})  # -> 2
```

#### Tipo de dato

```
tipo("hola") # -> "cadena"
```

#### Cadena

```
cadena(10) # -> "10"
```

# Funciones graficas internas

#### dibujar_texto

```
dibujar_texto("hola mundo", x, y, tamano_de_fuente);
# Color opcional
dibujar_texto("hola mundo", x, y, 0xFF0000);
# Con transparencia
dibujar_texto("hola mundo", x, y, 0xFF0000F0);
```

#### dibujar_linea

```
dibujar_linea(x1, y1, x2, y2);
# Color opcional
dibujar_linea(x1, y1, x2, y2, 0xFF00FF);
# Con transparencia
dibujar_linea(x1, y1, x2, y2, 0xFF00FF0F);
```

#### dibujar_rectangulo

```
dibujar_rectangulo(x1, y1, x2, y2);
# Color opcional
dibujar_rectangulo(x1, y1, x2, y2, 0xFF00FF);
# Con transparencia
dibujar_rectangulo(x1, y1, x2, y2, 0xFF00FF0F);
```

#### dibujar_circulo

```
dibujar_circulo(x, y, radio);
# Color opcional
dibujar_circulo(x, y, radio, 0x00FF00);
# Con transparencia
dibujar_circulo(x, y, radio, 0x00FF00AA);
```