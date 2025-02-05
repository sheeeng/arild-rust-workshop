# Rust introductory workshop

This workshop will give an introduction to Rust through the slides and some
hands-on exercices. Slides for the workshop:
https://arild.github.io/rust-workshop

## Preparations

The following steps should be completed before the workshop:

1. Install Rust
   - Linux/Mac: https://www.rust-lang.org/tools/install *
   - Windows 10:
     https://forge.rust-lang.org/infra/other-installation-methods.html#other-ways-to-install-rustup
2. Clone project: `git clone https://github.com/arild/rust-workshop.git`
3. Run `cargo test` in project root folder. Code should compile and it's
   **expected that tests fail** (coding exercises will make them green)
4. Import project to IDE. Verify that code completion is available.
   - IntelliJ plugin: https://plugins.jetbrains.com/plugin/22407-rust
   - Visual Studio Code:
     https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
5. (Optional) Rust Rover - standalone IDE from JetBrains
   - Public preview: https://www.jetbrains.com/rust/
6. (Optional) Cargo watch - watch for file changes and runs the command again
    ```bash
    cargo install cargo-watch
    ```
   ```bash
   cargo watch -x "test partX" // x is part
   ```

- If you encounter `linker 'cc' not found` error while trying to install Rust,
  try to install build-essentials first:

```bash
sudo apt install build-essentials
```
