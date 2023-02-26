# Comprenem *Ownership*

Rust inclou una funcionalitat original anomenada propietat (*ownership*), que
té profundes implicacions en la resta del llenguatge. *Ownership* permet a Rust
garantir la seguretat de la memòria sense necessitat d'un recol·lector
d'escombraries (*garbage collector*). Així, és important que entenguem bé com
funciona *Ownership*. En aquest capítol tractarem el tema de la propietat i
d'altres característiques relacionades com ara préstec (*borrowing*), seccions
(*slices*) i com Rust guarda les dades a memòria.

> Nota del traductor: per facilitar la comprensió de conceptes tan importants
> com el de propietat (*ownership*), préstec (*borrowing*) i seccions
> (*slices*), faré servir sovint els mots originals.
