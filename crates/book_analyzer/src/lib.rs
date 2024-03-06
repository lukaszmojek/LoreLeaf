use rbook::{Ebook, Epub, Reader};
use rbook::epub::Toc;
use rbook::xml::Element;

#[derive(Debug)]
pub struct BookDetails {
    pub title: String,
    pub creators: String,
    _epub: Epub
}

impl BookDetails {
    pub fn create(book_path: String) -> Option<BookDetails> {
        let doc = rbook::Epub::new(book_path);

        if doc.is_err() {
            //TODO: Think how to handle errors
            return None;
        }

        let doc = doc.unwrap();

        let title = match doc.metadata().title() {
            None => {"Unknown".to_string()}
            Some(metadata) => {metadata.value().to_string()}
        };

        let creators = doc.metadata()
            .creators()
            .iter()
            .map(|&x| x.value())
            .collect();

        Some(BookDetails {
            title,
            creators,
            _epub: doc
        })
    }

    pub fn table_of_contents(&self) -> &Toc {
        self._epub.toc()
    }

    pub fn reader(&self) -> Reader {
        self._epub.reader()
    }

    pub fn get_chapter(&self, chapter_id: &str) -> &Element {
        let manifest = self._epub.manifest();
println!("{:?}", manifest.elements()[3].name());
println!("{:?}", manifest.elements()[3].value());
println!("{:?}", manifest.elements()[3].children());
        self._epub.manifest().by_id(chapter_id).unwrap()
    }
    //
    // pub fn get_book_contents(&mut self) -> Vec<&String> {
    //     let mut book_contents: Vec<&String> = vec![];
    //
    //     for content in self.file.spine.iter() {
    //         book_contents.push(content);
    //     }
    //
    //     book_contents
    // }
    //
    // pub fn get_spine(&self) -> Vec<String> {
    //     self.file.spine.clone()
    // }
}

impl PartialEq for BookDetails {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.creators == other.creators
    }
}

