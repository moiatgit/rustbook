# Comprenem l'*Ownership*

Rust inclou una característica original anomenada *ownership*, que traduirem
com a *pertinença*. Aquesta característica, té profundes implicacions en la
resta del llenguatge. El mecanisme de pertinença permet a Rust garantir la
seguretat de la memòria sense necessitat d'un recol·lector d'escombraries
(*garbage collector*). Per tant, és important que entenguem bé com funciona
*Ownership*. En aquest capítol tractarem el tema de la pertinença i d'altres
característiques relacionades com ara préstec (*borrowing*), seccions
(*slices*) i com Rust guarda les dades a memòria.

> Notes del traductor:
>
> En aquesta traducció faré servir sovint els mots originals sense traduir, per
> conceptes rellevants com ara el de pertinença (*ownership*), préstec
> (*borrowing*) i seccions (*slices*), amb la intenció de que els lectors
> estigueu familiaritzats amb aquests termes i n'associeu el significat precís,
> a banda de poder entendre els missatges d'error que generarà el compilador.
>
> He escollit com a traducció de *ownership* el mot *pertinença* en comptes del
> més habitual *propietat*, reservant aquest últim per traduir *property*.
