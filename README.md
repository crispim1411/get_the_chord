# Get the Chord
Projeto desenvolvido em Rust para obter o acorde resultante dadas suas notas.

## Utilização
### Por código
```rust
let notes = vec![
    Note::new(C, Normal),
    Note::new(E, Normal),
    Note::new(G, Normal),
    Note::new(B, Flat),
];

let chord = notes_to_chord(notes);

println!("Chord: {}", chord);
```

>  Chord: Cmaj7

### Por linha de comando

> cargo run C# F G# C
>
> C#maj7

---

# Aplicação da Teoria musical
Até o momento foram mapeados os seguintes acordes:

| Acordes Maiores | Intervalos |
| :------ | ------------ |
| Maior | T - 3M - 5J  |
| Maior com quinta aumentada | T - 3M - 5+ |
| Maior com sexta | T - 3M - 5J - 6M |
| Maior com sétima    | T - 3M - 5J - 7M |
| Dominante | T - 3M - 5J - 7m |
| Dominante com quinta aumentada | T - 3M - 5+- 7m |
| Maior com sétima e quinta aumentada | T - 3M - 5+ - 7M |

| Acordes Menores | Intervalos |
| :------ | ------------ |
| Menor | T - 3m - 5J  |
| Menor com sexta | T - 3m - 5J - 6M |
| Menor com sétima | T - 3m - 5J - 7m |
| Menor com sétima maior | T - 3m - 5J - 7m |
| Meio diminuto | T - 3m - 5d - 7m |
| Diminuto | T - 3m - 5d  |
| Diminuto com sétima | T - 3m - 5d - 7d |

| Acordes Suspensos | Intervalos |
| :------ | ------------ |
| Segunda suspensa | T - 2M - 5J  |
| Quarta suspensa | T - 4J - 5J  |

| Acordes Invertidos | Intervalos |
| :------ | ------------ |
| Maior 1ª inversão | 1 - 3m - 6m  |
| Maior 2ª inversão | 1 - 4J - 6M  |
| Menor 1ª inversão | 1 - 3M - 6M  |
| Menor 2ª inversão | 1 - 4J - 6m  |

## Exemplos

| Acordes Maiores | Notas | Intervalos |
| :------ | ----- | ------------ |
| C | C - E - G | T - 3M - 5J |
| Aaug | A - C - E# | T - 3M - 5+ |
| C6 | C - E - G - A | T - 3M - 5J - 6M |
| Emaj7 |  E - G# - B - D# | T - 3M - 5J - 7M |
| GaugM7 | G - B - D# - F# | T- 3M - 5+ - 7M | 
| Ebm(maj7) | Eb - Gb - Bb - D | T - 3m - 5J - 7M |
| Asus2 | A - B - E | T - 2M - 5J |
| F#sus4 | F# - G# C# | T - 4J - 5J |
| Ab/Eb | Eb - Ab - C | 1 - 4J - 6M |
| C#m/E | E - C# - G# | 1 - 3M - 6M | 

