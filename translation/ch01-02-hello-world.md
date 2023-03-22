## Hola Món!

Ara que tenim instal·lat Rust, és el moment d'escriure el nostre primer
programa. Quan aprenem un nou llenguatge, és tradicional escriure un petit
programa que mostri el text `Hola Món!` a la pantalla. Això és el que farem ara
mateix!

> Nota: Aquest llibre presuposa una certa familiaritat amb la línia de comandes.
> Rust no té cap requeriment respecte editors o altres eines, ni tampoc on es
> trobarà el nostre codi, per tant, si t'estimes més fer servir un entorn
> integrat de desenvolupament (IDE) en comptes de la línia de comandes,
> endavant. Hi ha molts IDEs que ofereixen un cert suport a Rust. Fes una
> ullada a la documentació del teu IDE pels detalls. L'equip Rust s'ha enfocat
> en oferir un gran suport als IDEs via `rust-analyzer`. Mira
> [Appendix D][devtools]<!-- ignore --> per més detalls.

### Creem un directori de projecte

Començarem creant un directori en el que guardarem el nostre codi Rust. A Rust
tant li fa on guardes el teu codi, però pels exercicis i projectes d'aquest
llibre, et recomanem que creis la carpeta *projects* en el directori *home* i deixar
allà tots els nostres projectes.

Obre un terminal i introdueix les següents comandes per crear el directori
*projects*, i un altre directori pel projecte "Hola Món!" dins de *projects*.

Per Linux, macOS i PowerShell a Windows, tecleja el següent:

```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

Per Windows CMD, tecleja:

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

### Escrivim i executem un programa Rust

A continuació, crearem un nou fitxer font anomenat *main.rs*. Els fitxers de
Rust sempre finalitzen amb l'extensió *.rs*. Si el nom del fitxer inclou més
d'una paraula, la convenció és fer servir un guió baix per separar-les. Per
exemple, és preferible fer servir *hola_mon.rs* que *holamon.rs*.

Ara, obre el fitxer
*main.rs* que acabes de crear i copia-hi el codi del llistat 1-1.

<span class="filename">Filename: main.rs</span>

```rust
fn main() {
    println!("Hola Món!");
}
```

<span class="caption">Llistat 1-1: Un programa que escriu `Hola Món!`</span>

Guarda el fitxer i torna al terminal dins la carpeta
*~/projects/hello_world*. A Linux o macOS, tecleja les següents comandes per
compilar i executar el fitxer:

```console
$ rustc main.rs
$ ./main
Hola Món!
```

A Windows tecleja la comanda `.\main.exe` en comptes de `./main`:

```powershell
> rustc main.rs
> .\main.exe
Hola Món!
```

Independentment del teu sistema operatiu, la cadena `Hola Món!` ha d'aparèixer
escrita al terminal. Si no veus aquesta sortida, revisa la secció
d'instal·lació a [“Troubleshooting”][troubleshooting]<!-- ignore --> per trobar
ajuda.

Si si que hi veus `Hola Món!`, felicitats! Acabes d'escriure oficialment un
programa en Rust. Això et converteix en un programador/programadora Rust.
Benvinguda/benvingut!

### Anatomia d'un programa en Rust

Revisem el nostre programa
“Hola Món!” en detall. La primera peça del puzle és:

```rust
fn main() {

}
```

Aquestes línies defineixen una funció anomenada `main`. La funció `main` és
especial: és sempre el primer codi que s'executa a tot programa Rust. Aquí, la
primera línia declara una funció `main` que no té paràmetres ni retorna res. Si
hi hagueren paràmetres, hi anirien entre els parèntesis `()`.

El codi de la funció està envoltat entre `{}`. Rust requereix claudàtors
envoltant el cos de les funcions. Es considera bon estil obrir claudàtor a la
mateixa línia en que es declara la funció, tot separant-lo amb un espai.

> Nota: Si vols mantenir el codi dels teus projectes dins de l'estil estàndard,
> fes servir l'eina de formatat automàtic anomenada `rustfmt`, que donarà
> format al teu codi (més a `rustfmt` en [Appendix D][devtools]<!-- ignore
> -->). L'equip de Rust ha inclòs aquesta eina dins la distribució estandard de
> Rust, de la mateixa manera que amb `rustc`. Per tant, hauria d'estar ja
> instal·lada al teu equip!

El cos de la funció `main` conté el següent codi:

```rust
    println!("Hola Món!");
```

Aquesta línia fa tota la feina d'aquest petit programa: escriu el text a pantalla.
Hi ha quatre detalls rellevants aquí:

En primer lloc, l'estil de Rust és indentar amb quatre espais (res de tabulador).

En segon lloc, `println!` crida una macro de Rust. Si es tractés d'una funció,
hauríem escrit `println` (sense `!`). Tractarem les macros de Rust en més
detall al capítol 19. De moment només ens cal saber que fent servir `!` estem
cridant a una macro en comptes d'una funció regular, i que les macros no sempre
segueixen les mateixes regles que les funcions.

En tercer lloc trobem el text `"Hola Món!"`. Passem aquesta cadena a `println!`
com a argument, i el text és escrit a la pantalla.

I en quart lloc, finalitzem la línia amb un punt i coma (`:`) que indica que
l'expressió ha finalitzat i la següent ja pot començar. La majoria de les
línies de codi en Rust, finalitzen amb punt i coma.

### La compilació i l'execució són passes separades

Examinem cada passa que hem realitzat per crear i executar un programa.

Abans d'executar un programa en Rust, hem de compilar-ho fent servir el compilador de Rust `rustc`, tot passant-li el nom del fitxer amb el codi font:

```console
$ rustc main.rs
```

Si coneixes C o C++, te n'hauràs adonat que és semblant a `gcc` o `clang`. Un cop compilat correctament, Rust genera un executable binari.

A Linux, macOS i PowerShell, es pot veure l'executable introduint la comanda `ls` al terminal:

```console
$ ls
main  main.rs
```

En Linux i macOS veurem dos fitxers. Amb PowerShell a Windows, veurem els mateixos tres fitxers que veuríem des de CMD. Amb CMD, des de Windows, escriuríem el següent:

```cmd
> dir /B %= l'opció /B indica que només es mostrin els noms dels fitxers =%
main.exe
main.pdb
main.rs
```

Hi trobem el fitxer amb el codi font amb l'extensió *.rs*, el fitxer executable
(*main.exe* a Windows, i *main* a la resta de plataformes), i, quan som a
Windows, un fitxer que conté informació de depuració, amb l'extensió *.pdb*. A
partir d'aquí, executarem el fitxer *main* o *main.exe* de la següent manera:

```console
$ ./main # o bé .\main.exe des de Windows
```

Si el *main.rs* és el teu programa “Hola Món!”, la línia anterior fa escriure `Hola Món!`
al teu terminal.

Si tens més familiaritat amb algun llenguatge dinàmic com ara Ruby, Python o JavaScript, no t'hauràs trobat amb la necessitat d'haver de compilar abans d'executar un programa.
Rust és un llenguatge compilat, el que significa que pots compilar el teu codi i lliurar l'executable a altres, de manera que el puguin executar sense haver de tenir Rust instal·lat.
Quan lliures algú un fitxer *.rb*, *.py* o *.js*, li caldrà tenir Ruby, Python o JavaScript (respectivament) instal·lat. Per contra, en aquests llenguatges només cal fer una comanda per compilar i executar el programa. En el disseny de llenguatges, tot té pros i contres.

Fer servir
`rustc` per compilar programes senzills, està bé. A mida que els nostres projectes es fan més grans, però, voldrem gestionar totes les opcions i facilitar-nos la compartició del nostre codi.
A continuació coneixerem l'eina Cargo, que ens ajudarà a escriure programes reals en Rust.

[troubleshooting]: ch01-01-installation.html#troubleshooting
[devtools]: appendix-04-useful-development-tools.md
