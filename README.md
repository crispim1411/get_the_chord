# Get the Chord
Projeto desenvolvido em Rust para obter o acorde resultante dadas suas notas.

## Utilização
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

---

# Aplicação da Teoria musical
Até o momento foram mapeados os seguintes acordes:

| Acorde            | Intervalos |
| :------           | ------------ |
| Tríade Maior      | T - 3M - 5J  |
| Tríade Menor      | T - 3m - 5J  |
| Tétrade Maior     | T - 3M - 5J - 7M |
| Tétrade Menor     | T - 3m - 5J - 7m |
| Tétrade Dominante | T - 3M - 5J - 7m |

## Exemplo

| Acorde            | Intervalos |
| :------           | ------------ |
| C         | C - E  - G  |
| Cm        | C - Eb - G  |
| Cmaj7     | C - E  - G - B  |
| Cm7       | C - Eb - G - Bb |
| C7        | C - E  - G - Bb |