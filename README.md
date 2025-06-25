# 🦀 Rust Learning Grind

Welcome to my Rust journey — a focused learning path to master Rust for Web3, Solana, and systems-level programming.

## 🚀 Why I'm Learning Rust

- To build performant and safe smart contracts on **Solana**
- To deepen my understanding of **memory safety, ownership, and lifetimes**
- To contribute to open-source projects in **Web3 and decentralized AI**

---
## 📁 Folder Structure
rust_practice/
├── Practices/
│ ├── rust_variables/
│ ├── rust_borrowing/
│ ├── rust_booleans/
│ ├── rust_functions/
│ └── ... more topics
├── Exercises/
│ └── guessing_game_exercise/
├── README.md
└── Cargo.toml
## resources ive been using :
https://www.w3schools.com/rust
i highly recommend this as the starting point before going for the documenations tutorials --this helped me haha
goated documentaion:https://www.bing.com/ck/a?!&&p=ee8856fd04e7e4f1f2337d7d2624084c37bdbef8a0f6cc99e6ba74073c616cc7JmltdHM9MTc1MDgwOTYwMA&ptn=3&ver=2&hsh=4&fclid=3ece6b07-31cf-60fe-1aa4-7d0530d3615e&psq=rust+documentation+pdf+solana&u=a1aHR0cHM6Ly9kb2NzLnJzL3NvbGFuYS9sYXRlc3Qvc29sYW5hLw&ntb=1

## 📅 Start Date: June 21, 2025

## 📚 Week 1: Core Rust Foundations

### ✅ Day 1: Setup + First Program
- [x] Installed Rust via `rustup`
- [x] Set up VS Code + WSL2
- [x] Created and ran first program with `cargo`
- [x] Wrote basic `assert_eq!` tests

```rust
fn main() {
    let x = 2 + 2;
    assert_eq!(x, 4);
    println!("All good, x = {}", x);
}
