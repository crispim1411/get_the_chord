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

## Denvolvimento
Até o momento foram mapeados os seguintes acordes:


### Triade maior: 
T - 3M - 5J

### Triade menor: 
T - 3m - 5J

### Tétrade maior: 
T - 3M - 5J - 7M

### Tétrade menor: 
T - 3m - 5J - 7m

### Tétrade dominante: 
T - 3M - 5J - 7m
