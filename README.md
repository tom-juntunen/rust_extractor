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

### Example 1
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

### Example 2
1. Update search query to "job 1:3"
2. Run `cargo run`
3. View results

JOB_1_3 translations:
1. His substance also was seven thousand sheep, and three thousand camels, and five hundred yoke of oxen, and five hundred she-asses, and a very great household; so that this man was the greatest of all the children of the east.	
2. And his substance was seven thousand sheep, and three thousand camels, and five hundred yoke of oxen, and five hundred she-asses, and very many servants; and this man was greater than all the children of the east.
3. His possessions also were seven thousand sheep, three thousand camels, five hundred yoke of oxen, five hundred female donkeys, and a very great household; so that this man was the greatest of all the children of the east.
4. And his possession was seven thousand sheep, and three thousand camels, and five hundred yoke of oxen, and five hundred she asses, and a family exceeding great: and this man was great among all the people of the east.
5. And of cattle he had seven thousand sheep and goats, and three thousand camels, and a thousand oxen, and five hundred she-asses, and a very great number of servants. And the man was greater than any of the sons of the east.

References
1. King James Version
2. Darby Bible
3. World English Bible
4. Douay-Rheims 1899 American Edition
5. Bible in Basic English

Avoid hitting the rate limit!
Failed to fetch data for Bible in Basic English: HTTP Status 429 Too Many Requests
Failed to fetch data for Darby Bible: HTTP Status 429 Too Many Requests
Failed to fetch data for Douay-Rheims 1899 American Edition: HTTP Status 429 Too Many Requests
Failed to fetch data for King James Version: HTTP Status 429 Too Many Requests
Failed to fetch data for World English Bible: HTTP Status 429 Too Many Requests
Failed to fetch data for Young's Literal Translation: HTTP Status 429 Too Many Requests
Failed to fetch data for Open English Bible, Commonwealth Edition: HTTP Status 429 Too Many Requests
Failed to fetch data for World English Bible, British Edition: HTTP Status 429 Too Many Requests
Failed to fetch data for Open English Bible, US Edition: HTTP Status 429 Too Many Requests
Failed to fetch data for Clementine Latin Vulgate: HTTP Status 429 Too Many Requests
Failed to fetch data for Protestant Romanian Corrected Cornilescu Version: HTTP Status 429 Too Many Requests

Make a visualization
```
python -m venv venv
venv\Scripts\activate
pip install -r requirements.txt
python .\viz.py
```

### Build a cargo release
`cargo build --release`

### Run the release
`./target/release/rust_extractor`

##### References
1. bible-api (https://bible-api.com/)
2. esv_ranges.py (https://gist.github.com/eykd/842200)
3. quran api (https://alquran.cloud/api)