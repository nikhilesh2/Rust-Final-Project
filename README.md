# RUST Project 
_Nik Singh and Alex Wreschnig_

## Build Instructions
1. Install Rust via rustup on Unix-like machines:
>>> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

_rustup will add the proper directories to your path, but you may want to temporarily add ~/.cargo/bin to your path for building Rust code without restarting._

2. Clone the repository from Github

3. Use crate to build the project:
>>> crate build --release

_note that the --release flag optimizes the code, making it more difficult to debug but also more performant. We saw immense performance penalties when Rust was built without the --release flag._

4. Execute the project:
>>> crate run --release