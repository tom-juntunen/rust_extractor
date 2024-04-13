### Rust Extractor
A data extractor that fetches JSON data from public APIs, stores it on disk, reads, sorts, and prints the data to console.

### Setup Instructions
1. Install Rust
2. Add Rust to PATH, example for Windows: `%USER_PROFILE%\.cargo\bin`
3. Test the code compilation: `cargo clippy`
4. Run the code: `cargo run`

### Version 1
1. Bible data using bible-api.com (https://bible-api.com/)
This version allows specifying only the following query parameters:
- single verse: "john 3:16"
- abbreviated book name: "jn 3:16"
- verse range: "romans+12:1-2"
- multiple ranges: "romans 12:1-2,5-7,9,13:1-9&10"

The purpose of this version is to show the variation of language and interpretation across all available translations.
Future version may include the ability to filter out or select for certain translations as well.
