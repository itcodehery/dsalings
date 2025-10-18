## WORK-IN-PROGRESS
## Contributions Welcome!

# dsa-lings

`dsa-lings` is a collection of small exercises to help you learn and practice Data Structures and Algorithms in Rust. Inspired by `Rustlings`, this project aims to provide a similar interactive learning experience where you can solve problems and get immediate feedback.

## Getting Started

To get started with `dsa-lings`, you'll need to have Rust and Cargo installed. If you don't have them, you can install them by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

1.  **Clone the repository:**
    ```sh
    git clone https://github.com/your-username/dsa-lings.git
    cd dsa-lings
    ```
    (Replace `https://github.com/your-username/dsa-lings.git` with the actual URL of your repository.)

2.  **Install `cargo-watch` (recommended):**
    `cargo-watch` is a tool that runs a Cargo command whenever your source code changes. This is highly recommended for a `Rustlings`-like experience, as it will automatically check your solutions as you save your files.
    ```sh
    cargo install cargo-watch
    ```

## How to Use

The exercises are located in the `src/exercises` directory. Each exercise typically involves completing a Rust function and making its associated tests pass.

1.  **Open an exercise:** Navigate to an exercise file, for example, `src/exercises/add.rs`.
2.  **Read the instructions:** Each exercise file will contain comments guiding you on what needs to be implemented.
3.  **Implement the solution:** Write your Rust code to solve the problem.
4.  **Save your file:** Upon saving, `cargo-watch` will automatically re-run checks.

## Running Exercises with `cargo-watch`

To get the interactive, "check-on-save" experience, run `cargo-watch` from the root of your `dsa-lings` directory:

```sh
cargo watch -x 'check --all-targets'
```

This command will:
*   Monitor your project for file changes.
*   Execute `cargo check --all-targets` every time a file is saved. `cargo check` compiles your code and reports any errors or warnings without producing an executable. The `--all-targets` flag ensures that all targets (bins, libs, tests, examples, benchmarks) are checked.

Keep this command running in a separate terminal window while you work on exercises. When you save a file, you'll immediately see if your changes introduce any compilation errors or warnings.

If you want to run tests specifically, you can use:

```sh
cargo watch -x 'test'
```

Or for a specific test:

```sh
cargo watch -x 'test add'
```

## Exercise Structure

Each exercise follows a general pattern:

*   **`src/exercises/<exercise_name>.rs`**: This file contains the primary function(s) you need to implement. It will often have `TODO` comments or placeholders that you need to fill in.
    Example: `src/exercises/add.rs`
    ```rust
    // exercises/add.rs
    // Make me pass the test!

    pub fn add(a: i32, b: i32) -> i32 {
        // TODO: Implement the addition function
        0 // Placeholder, make this return the correct sum
    }
    ```

*   **`src/exercises/<exercise_name>_test.rs`**: This file contains the unit tests for the corresponding exercise. Your goal is to modify the exercise file (`<exercise_name>.rs`) so that all tests in this file pass. You generally should not need to modify the `_test.rs` files unless specifically instructed.
    Example: `src/exercises/add_test.rs`
    ```rust
    // exercises/add_test.rs

    #[cfg(test)]
    mod tests {
        use super::super::add; // Import the add function from add.rs

        #[test]
        fn test_add_positive_numbers() {
            assert_eq!(add::add(2, 3), 5);
        }

        // ... other test cases
    }
    ```

*   **`src/exercises/mod.rs`**: This file acts as the module declaration for all exercises. When you create a new exercise, you'll need to declare it here (and its test file) as public modules.
    Example:
    ```rust
    pub mod add;
    pub mod add_test;
    pub mod arrays;
    // ... other exercises
    ```

## Contributing

If you'd like to contribute new exercises or improve existing ones, feel free to open a pull request!
