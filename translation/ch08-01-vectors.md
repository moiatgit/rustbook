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

### Using an Enum to Store Multiple Types

Vectors can only store values that are the same type. This can be inconvenient;
there are definitely use cases for needing to store a list of items of
different types. Fortunately, the variants of an enum are defined under the
same enum type, so when we need one type to represent elements of different
types, we can define and use an enum!

For example, say we want to get values from a row in a spreadsheet in which
some of the columns in the row contain integers, some floating-point numbers,
and some strings. We can define an enum whose variants will hold the different
value types, and all the enum variants will be considered the same type: that
of the enum. Then we can create a vector to hold that enum and so, ultimately,
holds different types. We’ve demonstrated this in Listing 8-9.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-09/src/main.rs:here}}
```

<span class="caption">Llistat 8-9: Defining an `enum` to store values of
different types in one vector</span>

Rust needs to know what types will be in the vector at compile time so it knows
exactly how much memory on the heap will be needed to store each element. We
must also be explicit about what types are allowed in this vector. If Rust
allowed a vector to hold any type, there would be a chance that one or more of
the types would cause errors with the operations performed on the elements of
the vector. Using an enum plus a `match` expression means that Rust will ensure
at compile time that every possible case is handled, as discussed in Chapter 6.

If you don’t know the exhaustive set of types a program will get at runtime to
store in a vector, the enum technique won’t work. Instead, you can use a trait
object, which we’ll cover in Chapter 17.

Now that we’ve discussed some of the most common ways to use vectors, be sure
to review [the API documentation][vec-api]<!-- ignore --> for all the many
useful methods defined on `Vec<T>` by the standard library. For example, in
addition to `push`, a `pop` method removes and returns the last element.

### Dropping a Vector Drops Its Elements

Like any other `struct`, a vector is freed when it goes out of scope, as
annotated in Listing 8-10.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-10/src/main.rs:here}}
```

<span class="caption">Llistat 8-10: Showing where the vector and its elements
are dropped</span>

When the vector gets dropped, all of its contents are also dropped, meaning the
integers it holds will be cleaned up. The borrow checker ensures that any
references to contents of a vector are only used while the vector itself is
valid.

Let’s move on to the next collection type: `String`!

[data-types]: ch03-02-data-types.html#data-types
[nomicon]: ../nomicon/vec/vec.html
[vec-api]: ../std/vec/struct.Vec.html
[deref]: ch15-02-deref.html#following-the-pointer-to-the-value-with-the-dereference-operator
