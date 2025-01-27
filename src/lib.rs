use book_types::{BookMetadata, Chapter};
use sha2::{Digest, Sha256};
use std::{cmp::min, env, fmt::Write, fs::{self, File}, io::{self, Read}, path::Path};
use regex::Regex;
use chrono::Utc;

mod book_types;

pub fn create_book_template_from_env(env_file_path: &str) {
    dotenv::from_path(env_file_path).expect("Failed to load .env file from the specified path");
    let metadata = populate_book_metadata();
    println!("{:?}", metadata);

    write_book_metadata_to_file(&metadata, "../light-writer-rs/books-templates/simulated_book_chapter.json").unwrap();
    text_to_json("../light-writer-rs/books-templates/simulated_book_chapter.txt", "../light-writer-rs/books-templates/simulated_book_chapter.json").unwrap();
}


pub fn generate_isbn(title: &str, authors: &str) -> String {
    // Get the current timestamp in a string format (e.g., "2025-01-10T12:00:00Z")
    let timestamp = Utc::now().to_rfc3339();

    // Concatenate the title, authors, and timestamp to create a unique string
    let input = format!("{}{}{}", title, authors, timestamp);

    // Create a SHA-256 hasher
    let mut hasher = Sha256::new();

    // Write the concatenated string to the hasher
    hasher.update(input);

    // Get the resulting hash (a 32-byte array)
    let result = hasher.finalize();

    // Convert the hash to a hexadecimal string and return the first 13 characters (as an ISBN length)
    let mut isbn = String::new();
    for byte in result.iter().take(13) {
        write!(&mut isbn, "{:02x}", byte).expect("Unable to write byte");
    }

    isbn
}

pub fn populate_book_metadata() -> BookMetadata {
    // Read environment variables and populate BookMetadata struct
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
    // Read the content of the text file
    let mut file = File::open(text_file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Regex pattern to match chapter labels like [ch-TheChapterOne], [ch-TheChapterTwo], etc.
    let re = Regex::new(r"\[ch-(.*?)\]").unwrap();

    // Read the existing JSON file if it exists
    let mut book = if Path::new(json_file_path).exists() {
        let mut existing_file = File::open(json_file_path)?;
        let mut existing_content = String::new();
        existing_file.read_to_string(&mut existing_content)?;
        serde_json::from_str::<BookMetadata>(&existing_content).unwrap_or_else(|_| BookMetadata::new())
    } else {
        BookMetadata::new()
    };

    // Initialize a global page counter
    let mut global_page_number = 1;

    // Parse the new chapters and append if not already present
    for caps in re.captures_iter(&content) {
        let chapter_name = &caps[1]; // Extract the chapter name (e.g., "TheChapterOne")
        let chapter_start = caps.get(0).unwrap().end(); // End of the [ch-TheChapterName] tag
        let next_chapter_pos = re.find(&content[chapter_start..]).map_or(content.len(), |mat| chapter_start + mat.start());

        let chapter_content = &content[chapter_start..next_chapter_pos];

        // Add page markers, avoiding word truncation and keeping sentence boundaries
        let mut paginated_content = String::new();
        let mut current_pos = 0;
        let mut page_number = global_page_number; // Start with the current global page number

        while current_pos < chapter_content.len() {
            let mut next_pos = min(current_pos + 2000, chapter_content.len());

            // Find the last period (.) before the 2000th character, to avoid cutting words
            if let Some(period_pos) = chapter_content[current_pos..next_pos].rfind('.') {
                next_pos = current_pos + period_pos + 1; // Include the period in the page
            }

            paginated_content.push_str(&chapter_content[current_pos..next_pos]);

            // Insert the page marker
            if next_pos < chapter_content.len() {
                paginated_content.push_str(&format!("\n[page {}]\n", page_number));
            }

            current_pos = next_pos;
            page_number += 1; // Increment the page number for the next chunk
        }

        // Update the global page number for the next chapter
        global_page_number = page_number;

        // Check if the chapter already exists
        if !book.content.iter().any(|ch| ch.chapter == chapter_name) {
            book.content.push(Chapter {
                chapter: chapter_name.to_string(),
                content: paginated_content.trim().to_string(),
                notes: String::new(),
                quotes: Vec::new(),
            });
        }
    }

    // Update the total chapters count
    book.total_chapters = book.content.len() as u8;

    // Write the updated BookMetadata instance back to the JSON file
    let json_file = File::create(json_file_path)?;
    serde_json::to_writer_pretty(json_file, &book)?;

    eprintln!("Total chapters updated to: {}", book.total_chapters);

    Ok(())
}

pub fn get_content_by_path(file_path: &str) -> String {
    // Step 1: Read the file contents into a string
    let mut file = fs::File::open(file_path)
        .expect("Failed to open the file");
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Failed to read the file contents");

    // Step 2: Parse the JSON string
    let json_data: BookMetadata = serde_json::from_str(&file_contents)
        .expect("Failed to parse the JSON");

    // Step 3: Extract the "content" field
    serde_json::to_string_pretty(&json_data.content).unwrap_or_default()
}

pub fn count_characters(input: &str) -> usize {
    input.chars().count()
}

pub fn msg_encrypt(key: &str, plaintext: &str) -> String {
    // Convert the key and plaintext into bytes
    let key_bytes = key.as_bytes();
    let key_length = key_bytes.len();
    let plaintext_bytes = plaintext.as_bytes();

    // Perform XOR encryption
    let encrypted_bytes: Vec<u8> = plaintext_bytes
        .iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key_bytes[i % key_length])
        .collect();

    hex::encode(encrypted_bytes)
}

pub fn msg_decrypt(key: &str, hex_encrypted_text: &str) -> String {
    // Convert the key into bytes
    let key_bytes = key.as_bytes();
    let key_length = key_bytes.len();

    // Convert the hex string to raw bytes
    let encrypted_bytes = hex::decode(hex_encrypted_text).expect("Failed to decode hex");

    // Perform XOR decryption (same as encryption)
    let decrypted_bytes: Vec<u8> = encrypted_bytes
        .iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key_bytes[i % key_length])
        .collect();

    // Convert the decrypted bytes back into a string
    String::from_utf8(decrypted_bytes).expect("Failed to convert to UTF-8")
}