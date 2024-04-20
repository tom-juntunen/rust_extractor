
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, Write, BufWriter};
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::path::{PathBuf, Path};
use csv::Writer;
use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;
use rust_bert::pipelines::sentence_embeddings::{SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType};
use tch::Device;

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
    chapter: i32,
    verse: i32,
    text: String,
}

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TranslationDetail {
    key: String,
    book_id: String,
    chapter: i32,
    verse: i32,
    text: String,
    name: String,
    embedding: Vec<f32>,
}

fn visit_dirs(dir: &PathBuf, verse_map: &mut HashMap<String, Vec<TranslationDetail>>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        println!("Visiting path: {:?}", path);  // Debugging statement to show current path
        if path.is_dir() {
            println!("Entering directory: {:?}", path);  // Debug when entering a directory
            visit_dirs(&path, verse_map)?;
        } else if path.is_file() {
            let file = File::open(&path)?;
            let reader = BufReader::new(file);
            let verse: BibleVerse = serde_json::from_reader(reader)?;
            println!("Loaded file: {:?}", path);  // Debug successful file load

            // Loop through all verses in the BibleVerse object
            let file_part_ref = format!("{}_{:03}", verse.verses[0].chapter, verse.translation_id);
            let contains_file_part = path.to_str()
                .map(|s| s.contains(&file_part_ref))
                .unwrap_or(false);
            let contains_reference = path.to_str()
                .map(|s| s.contains(&verse.verses[0].book_id))
                .unwrap_or(false);
           
            if contains_file_part && contains_reference {
                println!("Reference '{:?}' contains file part match '{}'.", path.to_str(), contains_file_part);  // Debug matching reference
                for verse_data in verse.verses.iter() {
                    let key = format!("{}_{:03}_{:03}", verse_data.book_id, verse_data.chapter,  verse_data.verse);
                    println!("Processing verse with key '{}'.", key);  // Debug each verse processed
                    let detail = TranslationDetail {
                        key: key.clone(),
                        book_id: verse_data.book_id.clone(),
                        chapter: verse_data.chapter,
                        verse: verse_data.verse,
                        text: verse_data.text.clone(),
                        name: verse.translation_name.clone(),
                        embedding: vec![],
                    };
                    verse_map.entry(key).or_default().push(detail);
                }
            } else {
                println!("Reference '{:?}' does not contain file part match '{}' or ref match '{}'.", path.to_str(), contains_file_part, contains_reference);  // Debug non-matching reference
            }
        }
    }
    Ok(())
}

fn process_verses(verse_map: &mut HashMap<String, Vec<TranslationDetail>>, translations_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // First, visit directories and populate the verse_map
    visit_dirs(translations_dir, verse_map)?;

    // Iterate through each translation in verse_map
    for (_key, translations) in verse_map.iter_mut() {
        let mut texts = Vec::new();
        let mut index_map = Vec::new();

        // Prepare batch for processing
        for (index, translation) in translations.iter_mut().enumerate() {
            let cleaned_translation = translation.text.replace('\n', " ").trim().to_string();
            texts.push(cleaned_translation);
            index_map.push(index);
        }

        // Generate embeddings for the batch
        if let Ok(embeddings) = generate_embeddings(&texts) {
            for (embedding, index) in embeddings.into_iter().zip(index_map) {
                translations[index].embedding = embedding;
            }
        } else {
            eprintln!("Failed to generate embeddings");
        }

        println!("Processed verses for key: {}", _key);
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

fn generate_embeddings(sentences: &[String]) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> {
    // Cuda not available natively in Rust

    // Create the model specifying the device
    let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
        .with_device(Device::Cpu)
        .create_model()?;

    // Encode the sentences
    let embeddings = model.encode(sentences)?;

    Ok(embeddings)
}


// Function to write the data to a CSV file
fn write_to_csv(verse_map: &HashMap<String, Vec<TranslationDetail>>) -> Result<(), Box<dyn std::error::Error>> {
    let path = "similarity_report.csv";
    let file_exists = Path::new(path).exists();
    
    let file = OpenOptions::new()
        .create(true) // Create the file if it does not exist.
        .append(true) // Append to the file if it exists.
        .open(path)?;

    let mut wtr = Writer::from_writer(BufWriter::new(file));

    // Write the header only if the file did not exist
    if !file_exists {
        wtr.write_record(["Verse Key", "Book ID", "Chapter", "Verse", "Translation 1", "Translation 2", "Similarity", "Text 1", "Text 2"])?;
    }

    // Iterate over your data and write each record, ensuring automatic quoting where necessary
    for (key, translations) in verse_map {
        for i in 0..translations.len() {
            for j in i + 1..translations.len() {
                let sim = cosine_similarity(&translations[i].embedding, &translations[j].embedding);
                wtr.write_record([
                    key,
                    &translations[i].book_id,
                    &format!("{:03}", &translations[i].chapter),
                    &format!("{:03}", &translations[i].verse),
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
    ];

    // let references = ["Job 3:1-26", "Job 4:1-21", "Job 5:1-27"];
    // let references = ["Job 3:1-13", "Job 3:14-26"];
    let references = [
        "Psalms: 1:1-6",
        "Psalms: 2:1-12",
        "Psalms: 3:1-8",
        "Psalms: 4:1-8",
        "Psalms: 5:1-12",
        "Psalms: 6:1-10",
        "Psalms: 7:1-17",
        "Psalms: 8:1-9",
        "Psalms: 9:1-20",
        "Psalms: 10:1-18",
        "Psalms: 11:1-7",
        "Psalms: 12:1-8",
        "Psalms: 13:1-6",
        "Psalms: 14:1-7",
        "Psalms: 15:1-5",
        "Psalms: 16:1-11",
        "Psalms: 17:1-15",
        "Psalms: 18:1-50",
        "Psalms: 19:1-14",
        "Psalms: 20:1-9",
        "Psalms: 21:1-13",
        "Psalms: 22:1-31",
        "Psalms: 23:1-6",
        "Psalms: 24:1-10",
        "Psalms: 25:1-22",
        "Psalms: 26:1-12",
        "Psalms: 27:1-14",
        "Psalms: 28:1-9",
        "Psalms: 29:1-11",
        "Psalms: 30:1-12",
        "Psalms: 31:1-24",
        "Psalms: 32:1-11",
        "Psalms: 33:1-22",
        "Psalms: 34:1-22",
        "Psalms: 35:1-28",
        "Psalms: 36:1-12",
        "Psalms: 37:1-40",
        "Psalms: 38:1-22",
        "Psalms: 39:1-13",
        "Psalms: 40:1-17",
        "Psalms: 41:1-13",
        "Psalms: 42:1-11",
        "Psalms: 43:1-5",
        "Psalms: 44:1-26",
        "Psalms: 45:1-17",
        "Psalms: 46:1-11",
        "Psalms: 47:1-9",
        "Psalms: 48:1-14",
        "Psalms: 49:1-20",
        "Psalms: 50:1-23",
        "Psalms: 51:1-19",
        "Psalms: 52:1-9",
        "Psalms: 53:1-6",
        "Psalms: 54:1-7",
        "Psalms: 55:1-23",
        "Psalms: 56:1-13",
        "Psalms: 57:1-11",
        "Psalms: 58:1-11",
        "Psalms: 59:1-17",
        "Psalms: 60:1-12",
        "Psalms: 61:1-8",
        "Psalms: 62:1-12",
        "Psalms: 63:1-11",
        "Psalms: 64:1-10",
        "Psalms: 65:1-13",
        "Psalms: 66:1-20",
        "Psalms: 67:1-7",
        "Psalms: 68:1-35",
        "Psalms: 69:1-36",
        "Psalms: 70:1-5",
        "Psalms: 71:1-24",
        "Psalms: 72:1-20",
        "Psalms: 73:1-28",
        "Psalms: 74:1-23",
        "Psalms: 75:1-10",
        "Psalms: 76:1-12",
        "Psalms: 77:1-20",
        "Psalms: 78:1-72",
        "Psalms: 79:1-13",
        "Psalms: 80:1-19",
        "Psalms: 81:1-16",
        "Psalms: 82:1-8",
        "Psalms: 83:1-18",
        "Psalms: 84:1-12",
        "Psalms: 85:1-13",
        "Psalms: 86:1-17",
        "Psalms: 87:1-7",
        "Psalms: 88:1-18",
        "Psalms: 89:1-52",
        "Psalms: 90:1-17",
        "Psalms: 91:1-16",
        "Psalms: 92:1-15",
        "Psalms: 93:1-5",
        "Psalms: 94:1-23",
        "Psalms: 95:1-11",
        "Psalms: 96:1-13",
        "Psalms: 97:1-12",
        "Psalms: 98:1-9",
        "Psalms: 99:1-9",
        "Psalms: 100:1-5",
        "Psalms: 101:1-8",
        "Psalms: 102:1-28",
        "Psalms: 103:1-22",
        "Psalms: 104:1-35",
        "Psalms: 105:1-45",
        "Psalms: 106:1-48",
        "Psalms: 107:1-43",
        "Psalms: 108:1-13",
        "Psalms: 109:1-31",
        "Psalms: 110:1-7",
        "Psalms: 111:1-10",
        "Psalms: 112:1-10",
        "Psalms: 113:1-9",
        "Psalms: 114:1-8",
        "Psalms: 115:1-18",
        "Psalms: 116:1-19",
        "Psalms: 117:1-2",
        "Psalms: 118:1-29",
        "Psalms: 119:1-176",
        "Psalms: 120:1-7",
        "Psalms: 121:1-8",
        "Psalms: 122:1-9",
        "Psalms: 123:1-4",
        "Psalms: 124:1-8",
        "Psalms: 125:1-5",
        "Psalms: 126:1-6",
        "Psalms: 127:1-5",
        "Psalms: 128:1-6",
        "Psalms: 129:1-8",
        "Psalms: 130:1-8",
        "Psalms: 131:1-3",
        "Psalms: 132:1-18",
        "Psalms: 133:1-3",
        "Psalms: 134:1-3",
        "Psalms: 135:1-21",
        "Psalms: 136:1-26",
        "Psalms: 137:1-9",
        "Psalms: 138:1-8",
        "Psalms: 139:1-24",
        "Psalms: 140:1-13",
        "Psalms: 141:1-10",
        "Psalms: 142:1-7",
        "Psalms: 143:1-12",
        "Psalms: 144:1-15",
        "Psalms: 145:1-21",
        "Psalms: 146:1-10",
        "Psalms: 147:1-20",
        "Psalms: 148:1-14",
        "Psalms: 149:1-9",
        "Psalms: 150:1-6"
    ];
    let desktop_path = dirs::desktop_dir().expect("Failed to get desktop directory");
    println!("{:?}", references);

    let client = Client::new();
    for reference in references.iter() {
        let mut verse_map: HashMap<String, Vec<TranslationDetail>> = HashMap::new();  // Reset here for each reference
        let translations_dir = desktop_path.join("bibles");
        if translations_dir.exists() {
            fs::remove_dir_all(&translations_dir)?;
        }
        fs::create_dir(&translations_dir)?;

        /*
        */
        
        // "Do not use this API to download an entire bible, one verse or chapter-at-a-time. That's absolutely nuts. Please get the data from the source." - Tim Morgan
        

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

                let filename = format!("{}_{:03}.json", verse.verses[0].chapter, verse.translation_id);
                let json_path = path.join(filename);

                let mut file = File::create(&json_path)?;
                let json_data = serde_json::to_string_pretty(&verse)?;
                file.write_all(json_data.as_bytes())?;
                println!("Bible verse data for {} saved to: {:?}", name, json_path);

            } else {
                println!("Failed to fetch data for {}: HTTP Status {}", name, response.status());
            }
        }

        // Fill the verse map for the current chapter
        process_verses(&mut verse_map, &translations_dir)?;
        
        // Write to CSV once after processing all translations for a reference
        if !verse_map.is_empty() {
            write_to_csv(&verse_map)?;
        }

        println!("Sleeping for 5 minutes...");
        thread::sleep(Duration::from_secs(5 * 60));

    }

    Ok(())
}