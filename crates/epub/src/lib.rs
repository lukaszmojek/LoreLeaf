mod metadata;
mod spine;

use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;
use spine::BookSpine;
use std::fs::File;
use std::io::Read;
use zip::ZipArchive;

use crate::metadata::BookMetadata;

fn read_epub(epub_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let epub_file = File::open("./data/moby-dick.epub")?;
    let mut archive = ZipArchive::new(epub_file)?;

    let opf_path = parse_container(&mut archive)?;
    parse_opf(&mut archive, &opf_path)?;

    // Further processing...
    Ok(())
}

fn parse_container(zip: &mut ZipArchive<File>) -> Result<String, Box<dyn std::error::Error>> {
    let mut container_file = zip.by_name("META-INF/container.xml")?;
    let mut contents = String::new();
    container_file.read_to_string(&mut contents)?;

    let mut reader = Reader::from_str(&contents);
    reader.trim_text(true);

    let mut txt = Vec::new();
    let mut buf = Vec::new();

    // Find the OPF file path in the container XML
    let mut opf_path = String::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            Ok(Event::Eof) => break,
            Ok(Event::Start(e) | Event::Empty(e)) => {
                println!("{:?}", e);
                if let b"rootfile" = e.name().as_ref() {
                    for attribute in e.attributes() {
                        let attr = attribute?;
                        if attr.key == QName(b"full-path") {
                            opf_path = String::from_utf8(attr.value.into_owned())?;
                            break;
                        }
                    }
                }
            }
            Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),
            _ => (),
        }
        buf.clear();
    }

    if opf_path.is_empty() {
        println!("ERR");
        Err("OPF file not found".into())
    } else {
        println!("OK");
        Ok(opf_path)
    }
}

// Assume `opf_path` is the path obtained from the previous step
fn parse_opf(
    zip: &mut ZipArchive<File>,
    opf_path: &str,
) -> Result<Book, Box<dyn std::error::Error>> {
    let opf_content = get_opf_content(zip, opf_path).unwrap_or_else(|err| {
        eprintln!("{:?}", err);
        "NONE".to_string()
    });

    let book = Book::create_from_opf(&opf_content);
    println!("{:?}", opf_content);

    Ok(book)
}

fn get_opf_content(
    zip: &mut ZipArchive<File>,
    opf_path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut opf_file = zip.by_name(opf_path)?;
    let mut contents = String::new();
    opf_file.read_to_string(&mut contents)?;

    Ok(contents)
}

struct Book {
    metadata: BookMetadata,
    spine: BookSpine,
}

impl Book {
    pub fn create_from_opf(opf_content: &String) -> Book {
        let metadata = BookMetadata::from_opf(opf_content);
        let spine = BookSpine::from_opf(opf_content);

        println!("{:?}", spine);

        Book { metadata, spine }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_container_should_return_path_to_opf() {
        let epub_file = File::open("./data/moby-dick.epub").unwrap();
        let mut archive = ZipArchive::new(epub_file).unwrap();

        let opf_path = parse_container(&mut archive);

        assert!(opf_path.is_ok());
        assert_eq!(opf_path.unwrap(), "OPS/package.opf")
    }

    #[test]
    fn parse_opf_should_return_book_with_correct_metadata() {
        let epub_file = File::open("./data/moby-dick.epub").unwrap();
        let mut archive = ZipArchive::new(epub_file).unwrap();

        let opf_path = parse_container(&mut archive).unwrap();
        let book = parse_opf(&mut archive, &opf_path).unwrap();

        let book_metadata = book.metadata;

        assert!(book_metadata.creator.is_some());
        assert_eq!(
            book_metadata.creator.unwrap(),
            "Herman Melville".to_string()
        );
        assert!(book_metadata.title.is_some());
        assert_eq!(book_metadata.title.unwrap(), "Moby-Dick".to_string());
        assert!(book_metadata.language.is_some());
        assert_eq!(book_metadata.language.unwrap(), "en-US".to_string());
        assert!(book_metadata.identifier.is_some());
        assert_eq!(
            book_metadata.identifier.unwrap(),
            "code.google.com.epub-samples.moby-dick-basic".to_string()
        );
        assert!(book_metadata.publisher.is_some());
        assert_eq!(
            book_metadata.publisher.unwrap(),
            "Harper & Brothers, Publishers".to_string()
        );
        assert!(book_metadata.rights.is_some());
        assert_eq!(book_metadata.rights.unwrap(),
            "This work is shared with the public using the Attribution-ShareAlike 3.0 Unported (CC BY-SA 3.0) license.".to_string());
    }

    #[test]
    fn parse_opf_should_return_book_with_correct_spine() {
        let epub_file = File::open("./data/moby-dick.epub").unwrap();
        let mut archive = ZipArchive::new(epub_file).unwrap();

        let opf_path = parse_container(&mut archive).unwrap();
        let book = parse_opf(&mut archive, &opf_path).unwrap();

        let spine = book.spine;

        assert_eq!(spine.items.len(), 144);
        assert_eq!(spine.items[0].id, "cover");
        assert_eq!(spine.items[143].id, "toc");
    }
}
