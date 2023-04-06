## El tipus secció

Les *seccions* (en anglès *slices*) permeten referenciar una seqüència contigua d'elements en una col·lecció, en comptes de tota la col·lecció.\ Uan secció és un tipus de referència i, per tant, no té pertinença.

Fem un petit exercici de programació: escriu una funció que prengui un text format per paraules separades per espais, i retorni la pimera paraula que trobi al text.
Si la funció no troba un espai al string, retornarà tota la cadena ja que tota ella serà una paraula.

Comencem veient com escriuríem la signatura de la funció sense fer ús de seccions, i així podrem entendre quin és el problema que resolen les seccions:

```rust,ignore
fn primera_paraula(s: &String) -> ?
```

La funció `primera_paraula` espera un `&String` com a paràmetre. No volem adquirir la pertinença d'aquest string, així que amb la referència ja està bé.\ Ara, però què és el que hauríem de retornar? En realitat, no disposem d'una manera de parlar sobre una *part* d'un string. No obstant, sempre podem retornar l'índex del final de la paraula, indicat per un espai. Intentem això al llistat 4-7.

<span class="filename">Fitxer: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:here}}
```

<span class="caption">Llistat 4-7: La funció `primera_paraula` que retorna el valor de la posició del paràmetre `String`</span>

Com que ens cal atravesar el `String` element a element i comprovar si la posició correspon o no a un espai, convertirem el nostre `String` a una seqüència (array) de bytes, fent servir el mètode
`as_bytes`.

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:as_bytes}}
```

A continuació, creem un iterador sobre l'array de bytes, fent servir el mètode `iter`:

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:iter}}
```

Veurem els iteradors amb més detall al [capítol 13][ch13]<!-- ignore -->.
De moment, simplement entenguem que `iter` és un mètode que retorna cada element d'una col·lecció i que `enumerate` transforma el resultat de `iter` de manera que cada element and returns each element formi part d'una tupla. El primer element de cada tupla retornada per
`enumerate` és l'índex, mentre que el segon correspon a la referència a l'element corresponent a l'array de bytes.
És una mica més adequat que haver d'anar calculant l'índex.

Com que el mètode `enumerate` retorna una tupla, podem fer servir patrons per desestructurar-la. Veurem més sobre patrons al [capítol
6][ch6]<!-- ignore -->. Al bucle `for` especifiquem el patró que té `i`
com a índex i `&item` com la referència al byte.
Donat que obtenim una referència a l'element des de `.iter().enumerate()`, hem de fer servir `&` al patró.

Dins del bucle `for` cerquem pel byte que representa l'espai tot usant la síntaxi literal. Si trobem un espai, retornem la seva posició.
Altrament retornem la longitud del string fent servir `s.len()`.

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:inside_for}}
```

Ara tenim una manera de trobar l'índex del final de la primera paraula dins el string. Encara hi ha, però, un problema. Estem retornant un valor `usize` que només té significat en el context de `&String`. En altres paraules, com que es tracta d'un valor separat del `String`, no hi ha cap garantia de que aquest encara sigui vàlid en el futur. Considerem el programa de lllistat 4-8 que fa servir la funció `primera_paraula` del llistat 4-7.

<span class="filename">Fitxer: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-08/src/main.rs:here}}
```

<span class="caption">Llistat 4-8: Guardem el resultat de la funció
`primera_paraula` i a continuació modifiquem el contingut del `String`</span>

Aquest programa compila sense cap error i permetrà l'ús de `paraula` un cop cridat
`s.clear()`. Donat que el valor de `paraula` no està connectat a l'estat de `s`, `paraual` encara conté el valor `4`. Podem fer servir el valor `4` amb la variable `s` per intentar obtenir la primera paraula, però això suposaria un error ja que el contingut de `s` ha canviat després d'assignar `4` a `paraula`.

Haver-se de preocupar de si el valor de `paraula` es torna invalid quan canvia el contingut de
`s` és tediós i és molt fàcil que ens equivoquem! Encara és més delicat gestionar aquests indexos si escrivim la funció `segona_paraula`. La seva signatura hauria de tenir el següent aspecte:

```rust,ignore
fn segona_paraula(s: &String) -> (usize, usize) {
```

Ara estarem controlant l'índex d'inici *i* l'índex de final, i disposem de més valor encara que hem calculat de les dades en un determinat estat però que no estan associades a aquest estat. Disposem de tres variables no relacionades que cal que mantinguem sincronitzades.

Per sort, Rust ens ofereix una solució a aquest problema: seccions de string.

### Seccions de Strings

Una *secció de string* és una referència a part d'un `String`, i té el següent aspecte:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-17-slice/src/main.rs:here}}
```

En comptes d'una referència a tot el `String`, `hola` és una referència a una porció d'aquest `String`, especificada en la part extra `[0..4]`. Per crear seccions, fem servir rangs entre claudàtors, tot especificant `[índex_inicial..índex_final]`,
on `índex_inicial` és la primera posició en la secció i `índex_final` is
és la posició després a la darrera de la secció. Internament, l'estructura de dades d'una secció emmagatzema la posició inicial i la longitud de la secció que es correspon amb `índex_final` menys `índex_inicial`. Així, en el cas de `let
mon = &s[5..8];`, `mon` seria una secció que conté un punter al byte de la posició 5 de `s` amb una longitud de `3`.

La figura 4-6 ho mostra en forma de diagrama.

<img alt="Tres taules: una taula que representa les dades de s, que apunta al byte a l'índex
0 de la taula del contingut del string &quot;Hola&quot; Món al montícle.
La tercera taula representa les dades de la secció mon, que té una longitud de 3 i apunta al byte en la posició 5 de la taula al monticle."
src="img/trpl04-06.svg" class="center" style="width: 50%;" />

<span class="caption">Figura 4-6: Secció de string que fa referència a una part d'un `String`</span>

Amb la sintaxi de rangs de Rust `..`, quan comencem per l'índex 0, podem estalviar-nos indicar-ho. En altres paraules, les següents seccions són iguals:

```rust
let s = String::from("hola");

let seccio = &s[0..2];
let seccio = &s[..2];
```

De la mateixa manera, quan la secció ha d'incloure el darrer byte del 
`String`, podem estalviar-nos indicar-ho:

```rust
let s = String::from("hola");

let longitud = s.len();

let seccio = &s[3..longitud];
let seccio = &s[3..];
```

Fins i tot, podem no indicar cap dels dos indexos si volem una secció de tot el string:

```rust
let s = String::from("hola");

let longitud = s.len();

let seccio = &s[0..longitud];
let slice = &s[..];
```

> Nota: Els indexos indicats a un rang d'una secció de string han d'apuntar a
> un caràcter UTF-8 vàlid. Si intentem crear una secció en mig d'un caràcter
> multibyte, el programa finalitzarà amb un error. Donat que és només una
> introducció a les seccions, suposarem que el text del string està format
> només per caràcters ASCII. Tractarem amb més detall la gestió de UTF-8 a la
> secció [“Emmagatzemant texts en String codificats amb UTF-8”][strings]<!-- ignore -->
> del capítol 8.

Amb tota aquesta informació en ment, tornem a escriure 
`primera_paraula` de manera que ara retorni una secció de string.
El tipus que representa una “secció de string” s'escriu  `&str`:

<span class="filename">Fitxer: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-18-first-word-slice/src/main.rs:here}}
```

Obtenim l'índex del final de la paraula de la mateixa manera que ho fèiem al llistat 
4-7, tot cercant per la primera ocurrència d'un espai.
Un cop trobat un espai, retornem una secció de string fent servir l'inici del string i l'índex de l'espai trobat, com a
inici i final de la secció.

Ara, en cridar 
 `primera_paraula`, obtenim un valor que està associat al contingut del string.
Aquest valor es composa d'una referència a l'inici de la secció i el nombre d'elements de la secció.

També funcionarà retornar una secció per la funció `segona_paraula`:

```rust,ignore
fn segona_paraula(s: &String) -> &str {
```

Així, disposem d'una utilitat molt més robusta ja que el compilador se
n'encarregarà de garantir que les referències a `String` es mantinguin vàlides.
Recordem l'error del programa al llistat 4-8, en el que obteníem l'índex al
final de la primera paraula però que tot seguit buidàvem el string de manera
que el nostre índex era invàlid? Aquell codi era incorrecte des d'un punt de
vista lògic però el compilador no mostrava cap error. El problema acavaria
apareixen més tard quan intentèssim fer servir l'índex de la primera paraula en
el string buidat. Les seccions fan que aquest tipus d'error sigui impossible i
ens permeten saber molt abans que el nostre codi presenta aquest tipus de
problemes. La versió de la funció `primera_paraula` que fa servir seccions,
generarà un error de compilació:

<span class="filename">Fitxer: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-19-slice-error/src/main.rs:here}}
```

Aquest és l'error de compilació:

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-19-slice-error/output.txt}}
```


On diu `cannot borrow s as mutable because it is also borrowed as immutable`
ens està dient exactament això, que no és possible fer un prèstec com a mutable
ja que ja està sent prestat com a inmutable. El missatge d'error encara és més
informatiu i ens indica que en quina part de la línia 16 s'està produïnt el
prestec immutable, en quina de la 18 s'intenta el mutable i finalment en quina
de la 20 s'està encara intentant fer ús del prèstec inmutable de la línia 16.

Les regles de pertinença ens recorden que si tenim una referència immutable a
alguna dada, no podem treure'n una referència mutable. Donat que `clear`
necessita modificar el `String`, necessita obtenir-ne una referència mutable.
`println!` un cop cridat a `clear` fa servir la referència a `paraula`, de
manera que la referència inmutable ha de ser encara disponible en aquest punt.
Rust no permet la referència mutable en `clear` i a l'hora la referència
inmutable a `word` i, per tant, la compilació falla. Així, Rust no només ha fet
que la nostra funció sigui més fàcil de fer servir, sinó que a més ha eliminat
tota una família d'errors en temps de compilació!

<!-- Old heading. Do not remove or links may break. -->
<a id="string-literals-are-slices"></a>

#### Els literals de string com a seccions

Recall that we talked about string literals being stored inside the binary. Now
that we know about slices, we can properly understand string literals:

```rust
let s = "Hello, world!";
```

The type of `s` here is `&str`: it’s a slice pointing to that specific point of
the binary. This is also why string literals are immutable; `&str` is an
immutable reference.

#### String Slices as Parameters

Encara podem fer una millora més a `primera_paraula`, sabent que podem crear seccions tant de literals com de `String`.
Recordem l'antiga signatura:

```rust,ignore
fn primera_paraula(s: &String) -> &str {
```

Un Rustacean més experimentat hauria escrit aquesta signatura de la manera indicada al llistat 
4-9 ja que així permetria usar la mateixa funció tant amb valors de tipus 
 `&String` com amb `&str`.

```rust,ignore
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-09/src/main.rs:here}}
```

<span class="caption">Llistat 4-9: Millora de la funció `primera_paraula` fent
servir una secció de string com a tipus del paràmetre `s`</span>

Si tenim una secció de string, podem passar-la directament. Si tenim un 
`String`, podem passar una secció del 
`String` o bé una referència al `String`. Aquesta flexibilitat s'aprofita de 
la *coerció deref* o 
*deref coercions*, una característica que estudiarem a la secció 
[“Coercions implícites Deref en funcions i mètodes”][deref-coercions]<!--ignore--> del capítol 15.

Fer que les nostres funcions prenguin una secció de string en comptes d'una referència a 
`String`
les fa més generals i usables, sense pèrdua de funcionalitat:

<span class="filename">Fitxer: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-09/src/main.rs:usage}}
```

### Altres seccions

Les seccions de strings són, com és evident, específiques dels strings. Hi ha, però, un tipus de secció més general. Considerem el següent array:

```rust
let a = [1, 2, 3, 4, 5];
```

De la mateixa manera que podem voler referir-nos a part d'un string, també podem voler referir-nos a part d'un array.
Ho podem fer de la següent manera:

```rust
let a = [1, 2, 3, 4, 5];

let seccio = &a[1..3];

assert_eq!(seccio, &[2, 3]);
```

Aquesta secció té com a tipus `&[i32]`. Funciona de la mateixa manera que les
seccions de strings, tot guardant una referència al primer element i la
longitud. Farem servir aquest tipus de segments per tot tipus de col·leccions,
cosa que estudiarem més endavant quan parlem de vectors al capítol 8.

## Conclusions

Els conceptes de pertinença (*ownership*), préstec (*borrowing*) i seccions
(*slices*), garanteix la seguretat de memòria en temps de compilació als
programes en Rust. El llenguatge Rust ens ofereix control sobre l'ús de memòria
com ho fan altres llenguatges de programació, però disposar de propietari de
les dades permet eliminar-les quan el propietari surt de l'abast del programa,
de manera que no ens calgui escriure cap codi addicional per fer-ho nosaltres.

La pertinença afecta la manera en que funciona una gran part de Rust, per tant,
seguirem parlant d'aquests conceptes en profunditat, durant la resta del
llibre. Passem ara al capítol 5, i veiem com podem agrupar peces de dades
juntes en una `struct`.

[ch13]: ch13-02-iterators.html
[ch6]: ch06-02-match.html#patterns-that-bind-to-values
[strings]: ch08-02-strings.html#storing-utf-8-encoded-text-with-strings
[deref-coercions]: ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods
