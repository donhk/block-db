#[derive(Clone)]
pub struct Document {
    pub(crate) filename: String,
    pub(crate) hash: String,
    pub(crate) chunks: Vec<String>,
    pub(crate) bytes_read: Vec<usize>,
    pub(crate) parity_chunks: usize,
}

impl Document {
    pub fn new(filename: String, hash: String, chunks: Vec<String>, bytes_read: Vec<usize>, parity_chunks: usize) -> Self {
        Document {
            filename,
            hash,
            chunks,
            bytes_read,
            parity_chunks,
        }
    }
}

pub mod documents_db {
    use std::collections::HashMap;
    use crate::db::files_db::Document;

    static mut DOCUMENTS: Option<HashMap<String, Document>> = None;

    pub fn insert(document: Document) {
        unsafe {
            let documents = DOCUMENTS.get_or_insert_with(HashMap::new);
            documents.insert(document.filename.clone(), document);
        }
    }

    pub fn get(filename: &str) -> Option<Document> {
        unsafe {
            DOCUMENTS.as_ref()?.get(filename).cloned()
        }
    }

    pub fn get_map() -> Option<HashMap<String, Document>> {
        unsafe {
            DOCUMENTS.clone()
        }
    }
}