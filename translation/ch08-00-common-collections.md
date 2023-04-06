# Col·leccions comuns

La biblioteca estàndard de Rust inclou un bon nombre de estructures anomenades
*col·leccions*. La majoria dels altres tipus de dades representen un valor
concret. En el cas de les col·leccions, però, aquestes poden contenir múltiples
valors.
A diferència dels tipus array i tupla, inclosos en el llenguatge, les dades
guardades en aquestes col·leccions apunten a valors en el monticle. Això vol
dir que la quantitat de dades que conté una col·lecció no necessàriament és
coneguda en temps de compilació i pot créixer o decréixer durant l'execució del
programa.
Cada col·lecció ofereix diferents característiques i costos, de manera que
escollir l'apropiada per cada situació concreta és una habilitat que haurem de
desenvolupar. En aquest capítol discutirem tres col·leccions molt freqüentment
usades en els programes en Rust:

* *vector*: permet emmagatzemar un nombre variable de valors un seguit de l'altre.
* *string*: és una col·lecció de caràcters. Ja hem mencionat abans el tipus
  `String`, però en aquest capítol en parlarem en profunditat.
* *hash map*: ens permet associar un valor a una clau. Es tracta d'una
  implementació particular d'una estructura més general anomenada *map*.

Per aprendre sobre els altres tipus de col·leccions que ofereix la biblioteca
estàndard, considera [la documentació][collections].

Veurem com crear i modificar vectors, strings i hash maps, així com allò que els fa especials.

[collections]: ../std/collections/index.html
