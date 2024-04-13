use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, Write, BufWriter};
use csv::Writer;
use std::collections::HashMap;
use std::path::PathBuf;
use strsim::normalized_damerau_levenshtein;
use reqwest::blocking::Client;
use rust_bert::pipelines::sentence_embeddings::{SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TranslationDetail {
    text: String,
    name: String,
    embedding: Vec<f32>,
}

fn visit_dirs(dir: &PathBuf, reference: &str, verse_map: &mut HashMap<String, Vec<TranslationDetail>>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            visit_dirs(&path, reference, verse_map)?;
        } else if path.is_file() {
            let file = File::open(&path)?;
            let reader = BufReader::new(file);
            let verse: BibleVerse = serde_json::from_reader(reader)?;

            // Loop through all verses in the BibleVerse object
            if let Some(verse_data) = verse.verses.iter().find(|_v| {
                let similarity = normalized_damerau_levenshtein(&verse.reference, reference);
                similarity >= FUZZY_MATCH_THRESHOLD
            }) {
                let key = format!("{}_{}_{}", verse_data.book_id, verse_data.chapter, verse_data.verse);
                // Append verse text and translation name as a TranslationDetail object to the verse_map
                let detail = TranslationDetail {
                    text: verse_data.text.clone(),
                    name: verse.translation_name.clone(),
                    embedding: vec![],  // Initialize with an empty vector; embeddings will be filled in later
                };
                verse_map.entry(key).or_default().push(detail);
            }
        }
    }
    Ok(())
}

fn process_verses(verse_map: &mut HashMap<String, Vec<TranslationDetail>>, reference: &str, translations_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    visit_dirs(translations_dir, reference, verse_map)?;

    for (_key, translations) in verse_map.iter_mut() {
        for translation in translations.iter_mut() {
            let cleaned_translation = translation.text.replace('\n', " ");
            match generate_embeddings(&cleaned_translation) {
                Ok(embedding) => translation.embedding = embedding,
                Err(e) => eprintln!("Failed to generate embeddings for {} due to {}", translation.name, e),
            }
        }
    }

    Ok(())
}

fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

fn magnitude(vec: &[f32]) -> f32 {
    vec.iter().map(|x| x * x).sum::<f32>().sqrt()
}

fn cosine_similarity(vec1: &[f32], vec2: &[f32]) -> f32 {
    let dot = dot_product(vec1, vec2);
    let mag1 = magnitude(vec1);
    let mag2 = magnitude(vec2);
    dot / (mag1 * mag2)
}

fn generate_embeddings(cleaned_translation: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2).create_model()?;
    let sentences = [cleaned_translation];
    let embeddings = model.encode(&sentences)?;
    Ok(embeddings[0].clone())
}

// Function to write the data to a CSV file
fn write_to_csv(verse_map: &HashMap<String, Vec<TranslationDetail>>) -> Result<(), Box<dyn std::error::Error>> {
    let path = "similarity_report.csv";
    let file = File::create(path)?;
    let mut wtr = Writer::from_writer(BufWriter::new(file));

    // Write the header
    wtr.write_record(["Verse Key", "Translation 1", "Translation 2", "Similarity", "Text 1", "Text 2"])?;

    // Iterate over your data and write each record, ensuring automatic quoting where necessary
    for (key, translations) in verse_map {
        for i in 0..translations.len() {
            for j in i + 1..translations.len() {
                let sim = cosine_similarity(&translations[i].embedding, &translations[j].embedding);
                wtr.write_record([
                    key,
                    &translations[i].name,
                    &translations[j].name,
                    &format!("{:.2}", sim),
                    &translations[i].text.replace('\"', "\"\""),  // Use char for single quote replacement
                    &translations[j].text.replace('\"', "\"\""),  // Use char for single quote replacement
                ])?;
            }
        }
    }

    // Ensure data is flushed and file is written
    wtr.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let reference = "matthew 5:12";
    let desktop_path = dirs::desktop_dir().expect("Failed to get desktop directory");
    let translations_dir = desktop_path.join("bibles");

    // "Do not use this API to download an entire bible, one verse or chapter-at-a-time. That's absolutely nuts. Please get the data from the source." - Tim Morgan
    let client = Client::new();
    for (id, name) in translations.iter() {
        let url = format!("https://bible-api.com/{}?translation={}", reference, id);
        let response = client.get(&url).send()?;

         if response.status().is_success() {
            let response_text = response.text()?;
            if let Ok(err) = serde_json::from_str::<ErrorResponse>(&response_text) {
                if err.error == "not found" {
                    println!("Translation '{}' for {} is not available.", name, reference);
                    continue;
                }
            }
            
            let verse: BibleVerse = serde_json::from_str(&response_text)?;

            let path = translations_dir.join(&verse.verses[0].book_id).join(id);
            fs::create_dir_all(&path)?;

            let filename = format!("{}_{}_{}_{}.json", verse.verses[0].chapter, verse.verses[0].verse, verse.reference.replace(':', "_"), verse.translation_id);
            let json_path = path.join(filename);

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&json_path)?;

            let json_data = serde_json::to_string_pretty(&verse)?;
            file.write_all(json_data.as_bytes())?;
            println!("Bible verse data for {} saved to: {:?}", name, json_path);
        } else {
            println!("Failed to fetch data for {}: HTTP Status {}", name, response.status());
        }
    }

    // Aggregate verses from all translation files
    let mut verse_map: HashMap<String, Vec<TranslationDetail>> = HashMap::new();
    
    // Process verses to fill verse_map with data and embeddings
    process_verses(&mut verse_map, reference, &translations_dir)?;
    write_to_csv(&verse_map)?;

    Ok(())
}
