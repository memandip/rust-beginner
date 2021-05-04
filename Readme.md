# Rust Beginner

## Steps
- install rust
  - `curl https://sh.rustup.rs -sSf | sh`
  - Installing Rust using rustup will also install cargo.
    - cargo (package manager for rust) 
- create new project
  - `cargo new hello-world`    
  - ```
      $ cd hello_world
      $ tree .
      .
      ├── Cargo.toml
      └── src
          └── main.rs

      1 directory, 2 files
    ```
- run `cargo build`
- run `./target/debug/rust-beginner`
  - output "Hello, world!"