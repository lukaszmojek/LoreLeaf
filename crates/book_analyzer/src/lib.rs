use epub::doc::{EpubDoc, NavPoint};
use std::fs::File;
use std::io::BufReader;

#[derive(Debug)]
pub struct BookDetails {
    pub title: String,
    pub author: String,
    file: EpubDoc<BufReader<File>>
}

impl BookDetails {
    pub fn create_with_epubrs(book_path: String) -> Option<BookDetails> {
        let doc = EpubDoc::new(book_path);

        if doc.is_err() {
            //TODO: Think how to handle errors
            return None;
        }

        let doc = doc.unwrap();

        //TODO: Improve metadata extraction from book, firstly by list of mappings, later can be improved
        let title = doc.mdata("title").unwrap_or_else(|| "Unknown".to_string());
        let author = doc.mdata("creator").unwrap_or_else(|| "Unknown".to_string());

        Some(BookDetails {
            title,
            author,
            file: doc,
        })
    }

    pub fn create_with_rbook(book_path: String) -> Option<BookDetails> {
        let doc = EpubDoc::new(book_path);

        if doc.is_err() {
            //TODO: Think how to handle errors
            return None;
        }

        let doc = doc.unwrap();

        //TODO: Improve metadata extraction from book, firstly by list of mappings, later can be improved
        let title = doc.mdata("title").unwrap_or_else(|| "Unknown".to_string());
        let author = doc.mdata("creator").unwrap_or_else(|| "Unknown".to_string());

        Some(BookDetails {
            title,
            author,
            file: doc,
        })
    }
    
    pub fn table_of_contents(&self) -> Vec<NavPoint> {
        self.file.toc.clone()
    }
    
    pub fn number_of_chapters(&self) -> usize {
        self.file.get_num_pages()
    }

    pub fn get_chapter(&mut self, index: usize) -> Option<(String, String)> {
        let chapter_nav_point = self.file.toc.get(index);

        if chapter_nav_point.is_none() {
            return None;
        }
        
        let chapter_path = chapter_nav_point.unwrap().clone().content;
        println!("{:?}", chapter_path);
        let chapter_content = self.file.get_resource_str(chapter_path.to_str().unwrap());
        
        chapter_content
    }

    pub fn get_page(&mut self, index: usize) -> Option<(Vec<u8>, String)> {
        // let was_page_changed = self.file.set_current_page(index);
        // 
        // if was_page_changed {
        //     return None;
        // }
        let moved = self.file.go_next();
        let moved = self.file.go_next();
        let moved = self.file.go_next();
        let moved = self.file.go_next();
        let moved = self.file.go_next();
        
        let current_page = match self.file.get_current() {
            None => {None}
            Some(content) => {Some(content)}
        };
        
        current_page 
    }

    pub fn get_book_contents(&mut self) -> Vec<&String> {
        let mut book_contents: Vec<&String> = vec![];
        
        for content in self.file.spine.iter() {
            book_contents.push(content);
        }
        
        book_contents
    }
    
    pub fn get_spine(&self) -> Vec<String> {
        self.file.spine.clone()
    }
}

impl PartialEq for BookDetails {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.author == other.author
    }
}

