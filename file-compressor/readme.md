# file-compressor
a simple file compressor in Rust using the flate2 crate. 

## guide
### creating a new rust project

Run the command to create a new Rust project:

```bash
cargo new file_compressor
cd file_compressor
```
### adding dependencies to Cargo.toml
Add the flate2 dependency:
```
[dependencies]
flate2 = "1.0.24"
```
### writing the file compression code
Replace the contents of src/main.rs with the provided Rust code.

### building and running the program
In your terminal, navigate to the project directory (file_compressor) and run:

```bash
cargo run source_file.txt target_file.gz
# cargo run popcorn.txt popcorn.gz
```
