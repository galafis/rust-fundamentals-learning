# 🦀 Rust Fundamentals Learning

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white) ![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)

---

## 🇧🇷 Aprendizado de Fundamentos em Rust

Repositório de estudo e prática dos fundamentos da linguagem **Rust**, desenvolvido como parte da formação **DIO (Digital Innovation One)**. Contém exercícios progressivos que cobrem desde a sintaxe básica até conceitos avançados como ownership, borrowing e concorrência.

### 🎯 Objetivo

Documentar a jornada de aprendizado em Rust, construindo uma base sólida nos conceitos fundamentais que tornam Rust uma linguagem única: segurança de memória em tempo de compilação, zero-cost abstractions e concorrência sem data races.

### 📂 Estrutura

```
rust-fundamentals-learning/
├── src/
│   └── main.rs          # Programa principal com exemplos
├── .gitignore
├── LICENSE
└── README.md
```

### 🚀 Como Executar

```bash
# Clone o repositório
git clone https://github.com/galafis/rust-fundamentals-learning.git
cd rust-fundamentals-learning

# Compile e execute
rustc src/main.rs -o main
./main

# Ou com Cargo (se houver Cargo.toml)
cargo run
```

### 📚 Conceitos Abordados

| Conceito | Descrição |
|----------|-----------|
| **Variáveis e Tipos** | `let`, `mut`, tipos primitivos (`u32`, `bool`, `&str`) |
| **Ownership** | Sistema de propriedade único do Rust |
| **Borrowing** | Referências imutáveis e mutáveis |
| **Lifetimes** | Anotações de tempo de vida |
| **Pattern Matching** | `match`, `if let`, desestruturação |
| **Error Handling** | `Result<T, E>`, `Option<T>`, operador `?` |

---

## 🇬🇧 Rust Fundamentals Learning

Study repository for **Rust** programming fundamentals, developed as part of the **DIO (Digital Innovation One)** formation. Contains progressive exercises covering basic syntax through advanced concepts like ownership, borrowing, and concurrency.

### 🎯 Goal

Document the Rust learning journey, building a solid foundation in the concepts that make Rust unique: compile-time memory safety, zero-cost abstractions, and data-race-free concurrency.

### 🚀 How to Run

```bash
git clone https://github.com/galafis/rust-fundamentals-learning.git
cd rust-fundamentals-learning
rustc src/main.rs -o main
./main
```

### 📚 Topics Covered

- Variables, mutability, and data types
- Ownership, borrowing, and lifetimes
- Pattern matching and enums
- Error handling with `Result` and `Option`
- Structs, traits, and generics
- Basic concurrency with threads

---

**Author:** Gabriel Demetrios Lafis  
**License:** MIT  
