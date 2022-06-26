# Rust Programming Language

![Rust is Awesome](/GitHub_Images/Rust%20is%20Awesome.png)

## List of Commands

Initialize the rust project if you want to configure the rust project in the current directory.

> `cargo init`

Run the src/main.rs file generates the output in target/debug/rust

> cargo run

```bash
Compiling rust v0.1.0 (/home/imskanand/Documents/Web Development/rust)
 Finished dev [unoptimized + debuginfo] target(s) in 0.31s
  Running`target/debug/rust`
 Hello, Maa!
```

Run the src/main.rs file generates the output in target/debug/rust

> cargo run --release

```bash
Compiling rust v0.1.0 (/home/imskanand/Documents/Web Development/rust)
    Finished release [optimized] target(s) in 0.26s
     Running `target/release/rust`
Hello, Maa!
```

## Naming conventions

Snake case is preferred for naming conventions in the rust programming language.

Example: `myAge -> my_age`

During printing the format of the output must be of string literals.

Example: `print!("{}", my_age);`
