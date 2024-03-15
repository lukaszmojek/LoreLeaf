mod metadata;
mod spine;

use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;
use spine::BookSpine;
use std::borrow::Borrow;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
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
    manifest: BookManifest,
}

/// Struct to represent all items from the manifest
struct BookManifest {
    /// Each epub needs to have a list of manifest items
    items: Vec<Rc<ManifestItem>>,
}

impl BookManifest {
    pub fn from_opf(opf_content: &String) -> BookManifest {
        let mut reader = Reader::from_str(opf_content);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut manifest_items: Vec<Rc<ManifestItem>> = vec![];

        while let Ok(event) = reader.read_event_into(&mut buf) {
            match event {
                Event::Start(ref e) | Event::Empty(ref e) => {
                    if let b"item" = e.name().as_ref() {
                        BookManifest::recreate_manifest_entry(e, &mut manifest_items);
                    }
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }

        BookManifest {
            items: manifest_items,
        }
    }

    //TODO: Consider moving to ManifestItem
    fn recreate_manifest_entry(
        e: &quick_xml::events::BytesStart<'_>,
        manifest_items: &mut Vec<Rc<ManifestItem>>,
    ) {
        let mut id = "".to_string();
        let mut href = "".to_string();
        let mut media_type = "".to_string();

        for attribute_result in e.attributes() {
            let attribute = attribute_result.unwrap();
            match attribute.key {
                QName(b"id") => {
                    id = String::from_utf8(attribute.value.into_owned()).unwrap();
                }
                QName(b"href") => {
                    href = String::from_utf8(attribute.value.into_owned()).unwrap();
                }
                QName(b"media-type") => {
                    media_type = String::from_utf8(attribute.value.into_owned()).unwrap();
                }
                _ => {}
            }
        }

        let manifest_item = ManifestItem {
            id,
            href,
            media_type,
        };
        manifest_items.push(Rc::new(manifest_item))
    }

    /// Search for an item in the manifest by part of its id or href.
    /// Returns the first item that matches the search.
    pub fn search_for_item(&self, query: &str) -> Option<Rc<ManifestItem>> {
        for item in &self.items {
            if item.id.contains(query) || item.href.contains(query) {
                return Some(item.clone());
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct ManifestItem {
    id: String,
    href: String,
    media_type: String,
}

impl PartialEq for ManifestItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.href == other.href && self.media_type == other.media_type
    }
}

impl Book {
    pub fn create_from_opf(opf_content: &String) -> Book {
        let manifest = BookManifest::from_opf(opf_content);
        let metadata = BookMetadata::from_opf(opf_content);
        let spine = BookSpine::from_opf_and_manifest(opf_content, manifest.borrow());

        Book {
            metadata,
            spine,
            manifest,
        }
    }
}

#[cfg(test)]
mod book_tests {
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
        let expected_cover_manifest_item = ManifestItem {
            id: "cover".to_string(),
            href: "cover.xhtml".to_string(),
            media_type: "application/xhtml+xml".to_string(),
        };
        let expected_toc_manifest_item = ManifestItem {
            id: "toc".to_string(),
            href: "toc.xhtml".to_string(),
            media_type: "application/xhtml+xml".to_string(),
        };

        let opf_path = parse_container(&mut archive).unwrap();
        let book = parse_opf(&mut archive, &opf_path).unwrap();

        let spine = book.spine;

        assert_eq!(spine.items.len(), 144);
        assert_eq!(spine.items[0].id, "cover");
        assert_eq!(
            *(spine.items[0].value.clone()),
            expected_cover_manifest_item
        );
        assert_eq!(spine.items[143].id, "toc");
        assert_eq!(
            *(spine.items[143].value.clone()),
            expected_toc_manifest_item
        );
    }

    #[test]
    fn parse_opf_should_return_book_with_correct_manifest() {
        let epub_file = File::open("./data/moby-dick.epub").unwrap();
        let mut archive = ZipArchive::new(epub_file).unwrap();

        let opf_path = parse_container(&mut archive).unwrap();
        let book = parse_opf(&mut archive, &opf_path).unwrap();

        let manifest = book.manifest;

        let first_item = manifest.items[0].as_ref();
        println!("{:?}", first_item);
        assert_eq!(manifest.items.len(), 151);
        assert_eq!(manifest.items[0].id, "font.stix.regular");
        assert_eq!(manifest.items[0].href, "fonts/STIXGeneral.otf");
        assert_eq!(manifest.items[0].media_type, "application/vnd.ms-opentype");
        assert_eq!(manifest.items[1].id, "font.stix.italic");
        assert_eq!(manifest.items[2].id, "font.stix.bold");
        assert_eq!(manifest.items[3].id, "font.stix.bold.italic");
        assert_eq!(manifest.items[4].id, "toc");
        assert_eq!(manifest.items[149].id, "xchapter_136");
        assert_eq!(manifest.items[150].id, "brief-toc");
    }
}

#[cfg(test)]
mod manifest_tests {
    use super::*;

    #[test]
    fn search_for_item_should_return_matching_item_when_queried() {
        let epub_file = File::open("./data/moby-dick.epub").unwrap();
        let mut archive = ZipArchive::new(epub_file).unwrap();

        let opf_path = parse_container(&mut archive).unwrap();
        let book = parse_opf(&mut archive, &opf_path).unwrap();

        let manifest = book.manifest;

        let toc_from_manifest = manifest
            .search_for_item("toc")
            .unwrap_or_else(|| panic!("toc was not found during search in manifest"));

        assert_eq!(toc_from_manifest.id, "toc");
        assert_eq!(toc_from_manifest.href, "toc.xhtml");
        assert_eq!(toc_from_manifest.media_type, "application/xhtml+xml");
    }
}
