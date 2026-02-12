# Rust Fundamentals Learning

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) ![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)

---

## 🇧🇷 Aprendizado de Fundamentos em Rust

Repositório de estudo e prática dos fundamentos da linguagem **Rust**, desenvolvido como parte da formação **DIO (Digital Innovation One)**. O programa em `src/main.rs` percorre os conceitos centrais da linguagem em ordem progressiva, cada seção com código funcional e testes unitários.

### Estrutura

```
rust-fundamentals-learning/
├── Cargo.toml
├── src/
│   └── main.rs          # Programa principal com exemplos e testes
├── .gitignore
├── LICENSE
└── README.md
```

### Como Executar

```bash
git clone https://github.com/galafis/rust-fundamentals-learning.git
cd rust-fundamentals-learning

# Compilar e executar
cargo run

# Rodar os testes
cargo test
```

### Conceitos Abordados

| Conceito | Descrição |
|----------|-----------|
| **Variáveis e Tipos** | `let`, `mut`, tuplas, arrays, tipos primitivos |
| **Ownership** | Transferência de propriedade, `move` semântico |
| **Borrowing** | Referências imutáveis (`&T`) e mutáveis (`&mut T`) |
| **Lifetimes** | Anotações de tempo de vida em funções e structs |
| **Structs e Enums** | Definição, métodos, `impl`, variantes com dados |
| **Pattern Matching** | `match`, `if let`, guards, desestruturação |
| **Error Handling** | `Result<T, E>`, `Option<T>`, erros customizados |
| **Traits e Generics** | Implementação de traits, trait bounds, funções genéricas |
| **Concorrência** | Threads, `Arc`, `Mutex`, soma paralela |

---

## 🇬🇧 Rust Fundamentals Learning

Study repository for **Rust** programming fundamentals, developed as part of the **DIO (Digital Innovation One)** track. The program in `src/main.rs` walks through core language concepts in order, with working examples and unit tests for each section.

### How to Run

```bash
git clone https://github.com/galafis/rust-fundamentals-learning.git
cd rust-fundamentals-learning

cargo run    # compile and run
cargo test   # run the test suite
```

### Topics Covered

- Variables, mutability, tuples, and arrays
- Ownership and move semantics
- Borrowing (immutable and mutable references)
- Lifetime annotations on functions and structs
- Structs, enums, and methods
- Pattern matching with `match`, `if let`, and guards
- Error handling with `Result`, `Option`, and custom error types
- Traits, default implementations, and generic functions
- Concurrency with threads, `Arc`, and `Mutex`

---

**Author:** Gabriel Demetrios Lafis
**License:** MIT
