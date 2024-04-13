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

### Example
1. Update search query to "isaiahh 22:3"
2. Run `cargo run`
3. View results

ISA_22_3 translations:
- All thy rulers fled away together, they were bound by the archers; all that were found of thee were bound together; they fled afar off. (American Standard Version (1901))
- All your rulers ... have gone in flight; all your strong ones have gone far away. (Bible in Basic English)
- All thy rulers have fled together, they are taken prisoners without the bow: all that are found of thee are made prisoners together; they were fleeing far off. (Darby Bible)
- All the princes are fled together, and are bound hard: all that were found, are bound together, they are fled far off. (Douay-Rheims 1899 American Edition)
- All thy rulers are fled together, they are bound by the archers: all that are found in thee are bound together, which have fled from far.  (King James Version)
- All your rulers fled away together. They were bound by the archers. All who were found by you were bound together. They fled far away.  (World English Bible)
- All your rulers fled away together. They were bound by the archers. All who were found by you were bound together. They fled far away.  (World English Bible, British Edition)

See similarity_report.csv for finding the most similar or different translations.
