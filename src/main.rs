use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, Write};
use std::collections::HashMap;
use std::path::PathBuf;
use strsim::normalized_damerau_levenshtein;

const FUZZY_MATCH_THRESHOLD: f64 = 0.80;

#[derive(Debug, Serialize, Deserialize)]
struct BibleVerse {
    reference: String,
    verses: Vec<Verse>,
    text: String,
    translation_id: String,
    translation_name: String,
    translation_note: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Verse {
    book_id: String,
    book_name: Option<String>,
    chapter: u32,
    verse: u32,
    text: String,
}

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: String,
}

fn visit_dirs(dir: &PathBuf, reference: &str, verse_map: &mut HashMap<String, Vec<(String, String)>>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            visit_dirs(&path, reference, verse_map)?;
        } else if path.is_file() {

            let file = File::open(&path)?;
            let reader = BufReader::new(file);
            let verse: BibleVerse = serde_json::from_reader(reader)?;

            // We now loop through all verses in the BibleVerse object
            if let Some(verse_data) = verse.verses.iter().find(|_v| {
                let similarity = normalized_damerau_levenshtein(&verse.reference, reference);
                similarity >= FUZZY_MATCH_THRESHOLD
            }) {
                let key = format!("{}_{}_{}", verse_data.book_id, verse_data.chapter, verse_data.verse);
                // Append both the verse text and translation name as a tuple to the verse_map
                verse_map.entry(key).or_default().push((verse_data.text.clone(), verse.translation_name.clone()));
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let translations = [
        ("cherokee", "Cherokee New Testament"),
        ("bkr", "Bible kralická"),
        ("asv", "American Standard Version (1901)"),
        ("bbe", "Bible in Basic English"),
        ("darby", "Darby Bible"),
        ("dra", "Douay-Rheims 1899 American Edition"),
        ("kjv", "King James Version"),
        ("web", "World English Bible"),
        ("ylt", "Young's Literal Translation"),
        ("oeb-cw", "Open English Bible, Commonwealth Edition"),
        ("webbe", "World English Bible, British Edition"),
        ("oeb-us", "Open English Bible, US Edition"),
        ("clementine", "Clementine Latin Vulgate"),
        ("almeida", "João Ferreira de Almeida"),
        ("rccv", "Protestant Romanian Corrected Cornilescu Version"),
    ];

    let reference = "isaiahh 22:3";
    let desktop_path = dirs::desktop_dir().expect("Failed to get desktop directory");
    let translations_dir = desktop_path.join("bibles");
    

    // Aggregate verses from all translation files
    let mut verse_map: HashMap<String, Vec<(String, String)>> = HashMap::new();
    visit_dirs(&translations_dir, reference, &mut verse_map)?;

    // Collect keys and sort them
    let mut keys: Vec<&String> = verse_map.keys().collect();
    keys.sort();

    // Display all translations for each verse, ordered by verse key
    for key in keys {
        println!("{} translations:", key);
        if let Some(translations) = verse_map.get(key) {
            for (translation, name) in translations {
                let cleaned_translation = translation.replace('\n', " ");
                println!("• {} ({})", cleaned_translation, name);
            }
        }
        println!();
    }

    Ok(())
}
