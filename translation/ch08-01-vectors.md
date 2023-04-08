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

De la mateixa manera que amb qualsevol variable, si volem canviar els valors, necessitem fer la variable mutable, fent servir la paraula clau 
 `mut`, tal i com vam veure al capítol 3. 
Els nombres que hi col·loquem són tots de tipus `i32` i, per tant, Rust infereix que el tipus del vector és `Vec<i32»` sense necessitat d'anotacions.

### Reading Elements of Vectors

There are two ways to reference a value stored in a vector: via indexing or
using the `get` method. In the following examples, we’ve annotated the types of
the values that are returned from these functions for extra clarity.

Listing 8-4 shows both methods of accessing a value in a vector, with indexing
syntax and the `get` method.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-04/src/main.rs:here}}
```

<span class="caption">Llistat 8-4: Using indexing syntax or the `get` method to
access an item in a vector</span>

Note a few details here. We use the index value of `2` to get the third element
because vectors are indexed by number, starting at zero. Using `&` and `[]`
gives us a reference to the element at the index value. When we use the `get`
method with the index passed as an argument, we get an `Option<&T>` that we can
use with `match`.

The reason Rust provides these two ways to reference an element is so you can
choose how the program behaves when you try to use an index value outside the
range of existing elements. As an example, let’s see what happens when we have
a vector of five elements and then we try to access an element at index 100
with each technique, as shown in Listing 8-5.

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-05/src/main.rs:here}}
```

<span class="caption">Llistat 8-5: Attempting to access the element at index
100 in a vector containing five elements</span>

When we run this code, the first `[]` method will cause the program to panic
because it references a nonexistent element. This method is best used when you
want your program to crash if there’s an attempt to access an element past the
end of the vector.

When the `get` method is passed an index that is outside the vector, it returns
`None` without panicking. You would use this method if accessing an element
beyond the range of the vector may happen occasionally under normal
circumstances. Your code will then have logic to handle having either
`Some(&element)` or `None`, as discussed in Chapter 6. For example, the index
could be coming from a person entering a number. If they accidentally enter a
number that’s too large and the program gets a `None` value, you could tell the
user how many items are in the current vector and give them another chance to
enter a valid value. That would be more user-friendly than crashing the program
due to a typo!

When the program has a valid reference, the borrow checker enforces the
ownership and borrowing rules (covered in Chapter 4) to ensure this reference
and any other references to the contents of the vector remain valid. Recall the
rule that states you can’t have mutable and immutable references in the same
scope. That rule applies in Listing 8-6, where we hold an immutable reference
to the first element in a vector and try to add an element to the end. This
program won’t work if we also try to refer to that element later in the
function:


```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-06/src/main.rs:here}}
```

<span class="caption">Llistat 8-6: Attempting to add an element to a vector
while holding a reference to an item</span>

Compiling this code will result in this error:


```console
{{#include ../listings/ch08-common-collections/listing-08-06/output.txt}}
```

The code in Listing 8-6 might look like it should work: why should a reference
to the first element care about changes at the end of the vector? This error is
due to the way vectors work: because vectors put the values next to each other
in memory, adding a new element onto the end of the vector might require
allocating new memory and copying the old elements to the new space, if there
isn’t enough room to put all the elements next to each other where the vector
is currently stored. In that case, the reference to the first element would be
pointing to deallocated memory. The borrowing rules prevent programs from
ending up in that situation.

> Note: For more on the implementation details of the `Vec<T>` type, see [“The
> Rustonomicon”][nomicon].

### Iterating over the Values in a Vector

To access each element in a vector in turn, we would iterate through all of the
elements rather than use indices to access one at a time. Listing 8-7 shows how
to use a `for` loop to get immutable references to each element in a vector of
`i32` values and print them.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-07/src/main.rs:here}}
```

<span class="caption">Llistat 8-7: Printing each element in a vector by
iterating over the elements using a `for` loop</span>

We can also iterate over mutable references to each element in a mutable vector
in order to make changes to all the elements. The `for` loop in Listing 8-8
will add `50` to each element.

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-08/src/main.rs:here}}
```

<span class="caption">Llistat 8-8: Iterating over mutable references to
elements in a vector</span>

To change the value that the mutable reference refers to, we have to use the
`*` dereference operator to get to the value in `i` before we can use the `+=`
operator. We’ll talk more about the dereference operator in the [“Following the
Pointer to the Value with the Dereference Operator”][deref]<!-- ignore -->
section of Chapter 15.

Iterating over a vector, whether immutably or mutably, is safe because of the
borrow checker's rules. If we attempted to insert or remove items in the `for`
loop bodies in Listing 8-7 and Listing 8-8, we would get a compiler error
similar to the one we got with the code in Listing 8-6. The reference to the
vector that the `for` loop holds prevents simultaneous modification of the
whole vector.

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
