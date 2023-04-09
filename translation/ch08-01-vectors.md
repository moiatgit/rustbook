## Emmagatzemar llistes de valors en vectors

La primera col·lecció que estudiarem és `Vec<T>`, també coneguda com *vector*.
Els vectors permeten emmagatzemar més d'un valor en una única estructura de
dades que col·loca cada valor un al costat de l'altre a memòria. Els vectors
només poden emmagatzemar valors del mateix tipus. Són útils quan tenim una
llista d'elements, com ara línies de text d'un fitxers o els preus dels
elements en una cistella de la compra.

### Creació d'un nou vector

Per crear un nou vector buit, cridem la funció `Vec::new` com es mostra al
llistat 8-1.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-01/src/main.rs:here}}
```

<span class="caption">Llistat 8-1: Creació d'un nou vector buit per valors de
tipus `i32`</span>

Donat que no hem inserit cap valor al vector, Rust no sap de quin tipus seran
els elements que pretenem emmagatzemar-hi. Per aquesta raó, ens ha calgut
especificar una anotació de tipus. Es tracta d'un punt important. Els vectors
estan implementats fent servir genèrics. Veurem els genèrics amb els nostres
propis tipus al capítol 10. De moment en tindrem prou sabent que un array de
tipus `Vec<T>` pot emmagatzemar qualsevol tipus. Quan creem un vector per
emmagatzemar un tipus concret, podem especificar aqust tipus entre els angles.
Al llistat 8-1 hem indicat a Rust que el vector `v` contindrà elements de tipus
`i32`.

Sovint crearem els vectors amb valors inicials i Rust serà capaç d'inferir de
quin tipus és `T`, de manera que serà infreqüent que hàgim d'especificar
aquesta anotació. Rust ens ofereix la macro `vec!` que crearà un nou vector amb
els valors que li indiquem. El llistat 8-2 crea un nou `Vec<i32>` que conté els
valors `1`, `2` i `3`. El tipus enter `i32` ja que aquest és el tipus per
defecte, com vam veure a la secció [“Tipus de dades”][data-types]<!-- ignore
--> del capítol 3.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-02/src/main.rs:here}}
```

<span class="caption">Llistat 8-2: Creació d'un nou vector amb valors inicials.</span>

Donat que hem fet la inicialització amb valors 
`i32`, Rust pot inferir que el tipus de `v`
és `Vec<i32>`, i l'anotació del tipus deixa de ser necessària.
Veiem ara com modificar un vector.

### Modificar un Vector

El llistat 8-3 ens mostra com crear un vector i a continuació afegir-hi
elements amb el mètode `push`.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-03/src/main.rs:here}}
```

<span class="caption">Llistat 8-3: Ús del mètode `push` per afegir valors a un
vector</span>

De la mateixa manera que amb qualsevol variable, si volem canviar els valors,
necessitem fer la variable mutable, fent servir la paraula clau `mut`, tal i
com vam veure al capítol 3. Els nombres que hi col·loquem són tots de tipus
`i32` i, per tant, Rust infereix que el tipus del vector és `Vec<i32»` sense
necessitat d'anotacions.

### Llegint els elements d'un vector

Hi ha dues maneres de referenciar un valor emmagatzemat a un vector: via
indexacio o fent servir el mdtode `get`. En els següents exemples, per ajudar
en la comprensió, anotarem els tipus dels valors retornats per aquestes
funcions.

El llistat 8-4 mostra els dos mètodes d'accés a un valor dins un vector, amb la sintaxi d'indexació i amb el mètode `get`.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-04/src/main.rs:here}}
```

<span class="caption">Llistat 8-4: Ús de la sintaxi d'indexació i el mètode
`get` per a accedir a un element dins d'un vector</span>

Alguns detalls a tenir en compte: fem servir el valor `2` com a índex del
tercer element perquè els vectors s'indexen amb un número i comencen amb el
zero. L'ús de `&` i `[]` ens permet referenciar a l'element que es troba a la
posició indicada per l'index. `Option<&T>` que podem fer servir amb `match`.

Rust ofereix aquestes dues maneres de referenciar un element per permetrens escollir com volem que el programa es comporti quan intentem fer servir un índex fora de rang. Veiem què passa quan intentem accedir a l'element amb índex 100 amb cadascuna de les dues tècniques.

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-05/src/main.rs:here}}
```

<span class="caption">Llistat 8-5: Intentant accedir a l'element amb índex 100 en un vector que conté 5 elements.</span>

En executar aquest codi, el primer mètode `[]` farà que el programa avorti
l'execució perquè fa referència a un element no existent. Aquest mètode és més
adequat quan volem que el programa finalitzi amb error si s'intenta accedir a
un element més enllà del final del vector.

> Nota: en el context de Rust, quan el programa avorta l'execució es fa servir
> sovint l'expressió *panics*. Per tant, és interessant que ens familiaritzem
> amb aquest terme.

Quan passem al mètode `get` un índex fora del vector, retorna `None` sense
avortar l'execució. Farem servir aquest mètode quan som conscients que l'accès
a una posició fora de rang pot passar de tant en tant, en circumstàncies
normals. En aquest cas, el programa inclourà lògica per gestionar les respostes
`Some(&element)` i `None`, tal i com vam veure al capítol 6. Per exemple,
l'índex podria venir d'una persona que introdueix un número. Si accidentalment
els usuaris entren un valor massa gran pel vector actual, i el programa obté un
`None`, podem informar els usuaris que el valor introduït està fora de rang i
convidar-lo a introduir un de nou. Això faria el nostre programa més agradable
per ser usat que simplement avortar l'execució quan els usuaris s'equivoquen en
teclejar.

Quan el programa té una referència vàlida, el *borrow checker* força les regles
de pertinença i prèstec que vam veure al capítol 4, per assegurar que aquesta
referència i qualsevol altra al contingut del vector, continua es mantenen
vàlides. Recordem la regla que diu que no podem tenir referències mutables i
immutables dins del mateix abast. Aquest regla és pertinent al llistat 8-6 on
tenim una referència inmutable al primer element del vector, i intentem afegir
un element al final. El programa no funcionarà si intentem tambe er referència
a aquest element més endavant a la funció.


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-06/src/main.rs:here}}
```

<span class="caption">Llistat 8-6: Intent d'afegir un element a un vector
mentre hi ha una referència a un dels seus elements. </span>

En compilar aquest codi, obtindrem el següent error:


```console
{{#include ../listings/ch08-common-collections/listing-08-06/output.txt}}
```

El missatge ens indica que a la línia 4 es produeix el prèstec inmutable de
l'element dins del vector. A la línia 6 es produeix l'intent de passar un
prèstec mutable, i finalment a la línia 8 intentem fer servir el prèstec
inmutable.


El codi en el llistat 8-6 semblaria que hauria de funcionar ja que afegir un element al final del vector no hauria d'afectar al primer element. L'error es justifica pel funcionament intern dels vectors. Quan afegim un element a un vector, si aquest ja no té espai per afegir el no element, caldrà reservar un nou espai de memòria on hi càpiguin tots els elements, copiar-hi els elements anteriors, i finalment alliberant la memòria anterior. Si Rust no generés un error en aquest cas, la referència al primer element estaria apuntant a una posició de memòria ja alliberada. Les regles de prèstec eviten que el programa es trobi en aquesta situació.

> Nota: Es poden consultar més detalls de l aimplementació del tipus `Vec<T>`
> al llibre [“The Rustonomicon”][nomicon].

### Iterant sobre els valors d'un vector

Per accedir cada element d'un vector, iterarem sobre tots ells en comptes de
fer servir índexos per accedir-los d'un en un. El llistat 8-7 mostra com fer
servir el bucle `for` per obtenir referències inmutables a cada element d'un
vector de valors `i32` i anar-los mostrant.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-07/src/main.rs:here}}
```

<span class="caption">Llistat 8-7: Mostrant cada element d'un vector fent
servir el bucle `for`</span>

Podem també iterar sobre referències mutables a cada element d'un vector mutable, quan ens interessi fer canvis als elements. El bucle `for` del llistat 8-8 afegirà `50` a cada element.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-08/src/main.rs:here}}
```

<span class="caption">Llistat 8-8: Iterant sobre referències mutables a
elements d'un vector.</span>

Per canviar els valors als que fa referència, ens cal fer servir l'operador de
dereferenciació `*` amb el que obtenim el valor a `i` abans que puguem
incrementar-ho amb l'operador `+=`. En parlarem més sobre l'operador de
derreferenciació a la secció [“Seguint el punter al valor amb l'operador de
derreferenciació”][deref]<!-- ignore --> del capítol 15.

Gràcies a les regles de prèstec, resulta segur iterar sobre un vector, ja sigui amb referències mutables o inmutables. Si intentem afegir o eliminar un element 
dins del cos del bucle dels llistats 8-7 i 8-8, obtindríem un error de compilació similar al que hem vist pel codi del llistat 8-6. La referència a un vector que 
manté el bucle `for` evita modificacions simultànies de tot el vector.

### Fent servir un Enum per emmagatzemar múltiples tipus

Els vectors només poden emmagatzemar dades del mateix tipus. Això pot representar un inconvenient ja que hi ha casos en els que ens pot ser necessari emmagatzemar diferents tipus.
Afortunadament, les variants dels enumerats estan definides sota el mateix tipus enumerat. Així, quan necessitem un tipus que representi elements de diferents tipus, podem fer ús d'un enum.

Per exemple, suposem que volem guardar valors d'una fila a un full de càlcul en el que algunes columnes contenen enters, d'altres reals, i d'altres strings.
Podem definir un enumerat pel que les seves variants mantinguin els diferents tipus, de manera que totes les variants seran considerades del mateix tipus: l'enum.
Així, podem definir un vector del tipus enum i, d'aquesta manera, disposar de valors de cada tipus.
El llistat 8-9 demostra aquesta tècnica.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-09/src/main.rs:here}}
```

<span class="caption">Llistat 8-9: Definint un `enum` per emmagatzemar valors
de diferents tipus en un vector </span>

Rust necessita conéixer quins tipus haurà d'emmagatzemar el vector en temps de
compilació, per poder saber exactament quanta memòria del monticle necessitarà
per emmagatzemar cada element. A més, hem d'explicitar quins tipus estaran
permesos al vector. Si Rust oferís la possibilitat d'emmagatzemar qualsevol
tipus a un vector, hi hauria la possiblitat de que un o més tipus causessin
problemes amb les operacions del vector. Amb l'ús d'enums i expressions match,
Rust pot garantir en temps de compilació, que tots els casos d'ús possibles es
maneguen tal i com s'explicava al capítol 6.

Aquesta tècnica de l'enum, només funcionarà si sabem exhaustivament tots els
tipus possibles que ens caldrà emmagatzemar al vector. En cas que no els
coneixem tots, podem fer servir un objecte *trait*, que veurem al capítol 17.

Ara que hem vist algunes de les més habituals maneres de fer servir els
vectors, seria convenient revisar [la documentació][vec-api]<!-- ignore --> per
conéixer la resta de mètodes definits sobre `<T>` a la llibreria estàndard. Per
exemple, a banda de  `push`, hi ha el mètode `pop` que elimina i retorna el
darrer element del vector.

### Eliminant un vector, s'eliminen els seus elements

De la mateixa manera que qualsevol altra `struct`, un vector s'allibera quan
surt de l'abast, com es mostra al llistat 8-10.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-10/src/main.rs:here}}
```

<span class="caption">Llistat 8-10: Mostrant on són alliberats el vector i els seus elements</span>

Quan el vector és alliberat, tots els elements que conté també són eliminats. És a dir, a l'exemple els enters continguts al vector queden alliberats.
El control·lador de prèstecs assegura que qualsevol referència als continguts
del vector seran usades només mentre el vector sigui vàlid.

Passem ara al següent tipus de col·lecció: `String`!

[data-types]: ch03-02-data-types.html#data-types
[nomicon]: ../nomicon/vec/vec.html
[vec-api]: ../std/vec/struct.Vec.html
[deref]: ch15-02-deref.html#following-the-pointer-to-the-value-with-the-dereference-operator
