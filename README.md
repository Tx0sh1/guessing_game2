# Guessing Game 🎯

A simple command-line number guessing game written in Rust. This was my first Rust project, built to learn the language fundamentals.

## How to Play

1.  The game will randomly select a secret number between 1 and 100.
2.  You have 4 attempts (lives) to guess the number.
3.  After each guess, you'll be told if your guess is too high or too low.
4.  Guess the number before you run out of lives to win!
5.  After a game ends, you can choose to play again.

## How to Run

1.  Ensure you have [Rust and Cargo installed](https://www.rust-lang.org/tools/install).
2.  Clone or download this project.
3.  Navigate to the project directory in your terminal.
4.  Run the game using Cargo:
    ```bash
    cargo run
    ```

## What This Project Uses

*   **Variables & Mutability:** To store game state like the player's guess and remaining lives.
*   **User Input:** Handling input from the command line.
*   **Control Flow:** `loop`, `if`, and `match` statements to control the game logic.
*   **Error Handling:** Converting and validating user input (string to number).
*   **The `rand` Crate:** To generate the secret random number.
*   **Nested Loops:** To manage multiple game sessions.

## Learning Goals

This project was built to understand core Rust concepts:
- Ownership, Borrowing, and Lifetimes
- The `Result` type for error handling
- Using external crates with Cargo
- Pattern matching with `match`

---
Built as a learning exercise with 🦀 Rust!