use serde::{Serialize, Deserialize};

// Define the structure of a Chapter
#[derive(Serialize, Deserialize, Debug)]
pub struct Chapter {
    pub chapter: String,
    pub content: String,
    pub notes: String,
    pub quotes: Vec<String>,
}

// Define the structure of the Book Metadata
#[derive(Serialize, Deserialize, Debug)]
pub struct BookMetadata {
    pub title: String,
    pub authors: String,
    pub production: String,
    pub subtitle: String,
    pub page_index: u32,
    pub isbn: String,
    pub publisher: String,
    pub publication_date: String,
    pub language: String,
    pub genres: Vec<String>,
    pub tags: Vec<String>,
    pub edition: String,
    pub description: String,
    pub table_of_contents: Vec<String>,
    pub cover_image_url: String,
    pub page_count: u32,
    pub notes: String,
    pub quotes: Vec<String>,
    pub references: Vec<String>,
    pub keywords: Vec<String>,
    pub modified_date: String,
    pub categories: Vec<String>,
    pub illustrator: String,
    pub editor: String,
    pub translator: String,
    pub dedication: String,
    pub acknowledgments: String,
    pub introduction: String,
    pub preface: String,
    pub appendices: Vec<String>,
    pub index_terms: Vec<String>,
    pub related_books: Vec<String>,
    pub resources: Vec<String>,
    pub format: String,
    pub content: Vec<Chapter>,
    pub total_chapters: u8,

}

impl BookMetadata {
    pub fn new() -> Self {
        BookMetadata {
            title: String::new(),
            authors: String::new(),
            production: String::from("The Digital Archive"),
            subtitle: String::new(),
            page_index: 0,
            isbn: String::new(),
            publisher: String::new(),
            publication_date: String::from("--/--/----"),
            language: String::new(),
            genres: vec!["".to_string(), "".to_string()],
            tags: vec!["".to_string()],
            edition: String::new(),
            description: String::new(),
            table_of_contents: Vec::new(),
            cover_image_url: String::new(),
            page_count: 0,
            notes: String::new(),
            quotes: Vec::new(),
            references: Vec::new(),
            keywords: Vec::new(),
            modified_date: String::new(),
            categories: Vec::new(),
            illustrator: String::new(),
            editor: String::new(),
            translator: String::new(),
            dedication: String::new(),
            acknowledgments: String::new(),
            introduction: String::new(),
            preface: String::new(),
            appendices: Vec::new(),
            index_terms: Vec::new(),
            related_books: Vec::new(),
            resources: Vec::new(),
            format: String::from("Blockchain"),
            content: Vec::new(),
            total_chapters: 0,

        }
    }
}
