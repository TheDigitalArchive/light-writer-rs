pub mod book_types;
use book_types::{BookMetadata, Chapter, Page};
use rand::Rng;
use sha2::{Digest, Sha256};
use std::{cmp::min, env, fmt::Write, fs::{self, File}, io::{self, Read}, path::Path};
use regex::Regex;
use chrono::Utc;


use aes::Aes256;
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn create_book_template_from_env(env_file_path: &str) {
    dotenv::from_path(env_file_path).expect("Failed to load .env file from the specified path");
    let metadata = populate_book_metadata();
    println!("{:?}", metadata);

    write_book_metadata_to_file(&metadata, "../light-writer-rs/books-templates/simulated_book_chapter.json").unwrap();
   text_to_json("../light-writer-rs/books-templates/simulated_book_chapter.txt", "../light-writer-rs/books-templates/simulated_book_chapter.json").unwrap();
}


pub fn generate_isbn(title: &str, authors: &str) -> String {
    let timestamp = Utc::now().to_rfc3339(); // format (e.g., "2025-01-10T12:00:00Z")
    let input = format!("{}{}{}", title, authors, timestamp);
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    let mut isbn = String::new();
    for byte in result.iter().take(13) {
        write!(&mut isbn, "{:02x}", byte).expect("Unable to write byte");
    }
    isbn
}

pub fn populate_book_metadata() -> BookMetadata {
    let metadata = BookMetadata {
        
        title: env::var("TITLE").unwrap_or_default(),
        authors: env::var("AUTHORS").unwrap_or_default(),
        production: env::var("PRODUCTION").unwrap_or("The Digital Archive".to_string()),
        subtitle: env::var("SUBTITLE").unwrap_or_default(),
        page_index: env::var("PAGE_INDEX").unwrap_or("0".to_string()).parse::<u32>().unwrap_or(0),
        isbn: generate_isbn(&env::var("TITLE").unwrap_or_default(),&env::var("AUTHORS").unwrap_or_default()),
        publisher: env::var("PUBLISHER").unwrap_or_default(),
        publication_date: env::var("PUBLICATION_DATE").unwrap_or("--/--/----".to_string()),
        language: env::var("LANGUAGE").unwrap_or_default(),
        genres: env::var("GENRES").unwrap_or_default().split(',').map(String::from).collect(),
        tags: env::var("TAGS").unwrap_or_default().split(',').map(String::from).collect(),
        edition: env::var("EDITION").unwrap_or_default(),
        description: env::var("DESCRIPTION").unwrap_or_default(),
        table_of_contents: env::var("TABLE_OF_CONTENTS").unwrap_or_default().split(',').map(String::from).collect(),
        cover_image_url: env::var("COVER_IMAGE_URL").unwrap_or_default(),
        page_count: env::var("PAGE_COUNT").unwrap_or("0".to_string()).parse::<u32>().unwrap_or(0),
        notes: env::var("NOTES").unwrap_or_default(),
        quotes: env::var("QUOTES").unwrap_or_default().split(',').map(String::from).collect(),
        references: env::var("REFERENCES").unwrap_or_default().split(',').map(String::from).collect(),
        keywords: env::var("KEYWORDS").unwrap_or_default().split(',').map(String::from).collect(),
        modified_date: Utc::now().to_rfc3339(),
        categories: env::var("CATEGORIES").unwrap_or_default().split(',').map(String::from).collect(),
        illustrator: env::var("ILLUSTRATOR").unwrap_or_default(),
        editor: env::var("EDITOR").unwrap_or_default(),
        translator: env::var("TRANSLATOR").unwrap_or_default(),
        dedication: env::var("DEDICATION").unwrap_or_default(),
        acknowledgments: env::var("ACKNOWLEDGMENTS").unwrap_or_default(),
        introduction: env::var("INTRODUCTION").unwrap_or_default(),
        preface: env::var("PREFACE").unwrap_or_default(),
        appendices: env::var("APPENDICES").unwrap_or_default().split(',').map(String::from).collect(),
        index_terms: env::var("INDEX_TERMS").unwrap_or_default().split(',').map(String::from).collect(),
        related_books: env::var("RELATED_BOOKS").unwrap_or_default().split(',').map(String::from).collect(),
        resources: env::var("RESOURCES").unwrap_or_default().split(',').map(String::from).collect(),
        format: env::var("FORMAT").unwrap_or("Blockchain".to_string()),
        content: Vec::new(),
        total_chapters: 0,
    };

    metadata
}

pub fn write_book_metadata_to_file(metadata: &BookMetadata, file_path: &str) -> io::Result<()> {
    let file = File::create(file_path)?;
    serde_json::to_writer_pretty(file, metadata)?;
    Ok(())
}

pub fn text_to_json(text_file_path: &str, json_file_path: &str) -> io::Result<()> {
    let mut file = File::open(text_file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let re = Regex::new(r"\[ch-(.*?)\]").unwrap();  // regex -> i.e [ch-TheChapterOne], [ch-TheChapterTwo], etc.
    let mut book = if Path::new(json_file_path).exists() {
        let mut existing_file = File::open(json_file_path)?;
        let mut existing_content = String::new();
        existing_file.read_to_string(&mut existing_content)?;
        serde_json::from_str::<BookMetadata>(&existing_content).unwrap_or_else(|_| BookMetadata::new())
    } else {
        BookMetadata::new()
    };
    let mut global_page_counter = 1;

    // Parse the new chapters and append if not already present
    for caps in re.captures_iter(&content) {
        let chapter_name = &caps[1]; // Extract the chapter name (e.g., "TheChapterOne")
        let chapter_start = caps.get(0).unwrap().end(); // End of the [ch-TheChapterName] tag
        let next_chapter_pos = re.find(&content[chapter_start..]).map_or(content.len(), |mat| chapter_start + mat.start());

        let chapter_content = &content[chapter_start..next_chapter_pos];

        let mut pages = Vec::new();
        let mut current_pos = 0;
        let mut page_number = global_page_counter;

        while current_pos < chapter_content.len() {
            let mut next_pos = min(current_pos + 2500, chapter_content.len());

            // Find the last period (.) before the 2500th character, to avoid cutting words
            if let Some(period_pos) = chapter_content[current_pos..next_pos].rfind('.') {
                next_pos = current_pos + period_pos + 1; // Include the period in the page
            }

            let page_content = chapter_content[current_pos..next_pos].to_string();

            pages.push(Page {
                page_number,
                content: page_content,
            });

            current_pos = next_pos;
            page_number += 1;
        }
        global_page_counter = page_number;

        // Check if the chapter already exists
        if !book.content.iter().any(|ch| ch.chapter == chapter_name) {
            book.content.push(Chapter {
                chapter: chapter_name.to_string(),
                pages,
                notes: String::new(),
                quotes: Vec::new(),
            });
        }
    }

    book.total_chapters = book.content.len() as u8;

    let json_file = File::create(json_file_path)?;
    serde_json::to_writer_pretty(json_file, &book)?;

    eprintln!("Total chapters updated to: {}", book.total_chapters);

    Ok(())
}

pub fn get_content_by_path(file_path: &str) -> String {
    let mut file = fs::File::open(file_path)
        .expect("Failed to open the file");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Failed to read the file contents");

    let json_data: BookMetadata = serde_json::from_str(&file_contents)
        .expect("Failed to parse the JSON");

    serde_json::to_string_pretty(&json_data.content).unwrap_or_default()
}

pub fn count_characters(input: &str) -> usize {
    input.chars().count()
}

pub fn light_msg_encryption(key: &[u8], json_file_path: &str) -> io::Result<String> {
    let mut file = File::open(json_file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let book_metadata: BookMetadata = serde_json::from_str(&content)?;
    let serialized_content = serde_json::to_string(&book_metadata)?;

    let mut rng = rand::rng();
    let mut iv = [0u8; 16];
    rng.fill(&mut iv);

    let cipher = Aes256Cbc::new_from_slices(key, &iv).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let encrypted_bytes = cipher.encrypt_vec(serialized_content.as_bytes());
    let encrypted_hex = hex::encode(encrypted_bytes);
    let iv_hex = hex::encode(iv);

    Ok(format!("{}:{}", iv_hex, encrypted_hex))
}

pub fn light_msg_decryption(key: &[u8], encrypted_message: &str) -> io::Result<BookMetadata> {
    let parts: Vec<&str> = encrypted_message.split(":").collect();
    if parts.len() != 2 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid encrypted message format"));
    }

    let iv_hex = parts[0];
    let encrypted_hex = parts[1];
    let iv = hex::decode(iv_hex).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let encrypted_data = hex::decode(encrypted_hex).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let cipher = Aes256Cbc::new_from_slices(key, &iv).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let decrypted_bytes = cipher.decrypt_vec(&encrypted_data).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let decrypted_str = String::from_utf8(decrypted_bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let book_metadata: BookMetadata = serde_json::from_str(&decrypted_str)?;

    Ok(book_metadata)
}