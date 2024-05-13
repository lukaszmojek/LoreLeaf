use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    fs::File,
    io::Read,
    path::PathBuf,
};

use quick_xml::{events::Event, name::QName, Reader};
use zip::ZipArchive;

use crate::{
    manifest::BookManifest,
    metadata::BookMetadata,
    spine::BookSpine,
    table_of_contents::{
        table_of_contents::TableOfContents, table_of_contents_item::TableOfContentsItem,
    },
};

pub struct EBook {
    pub metadata: BookMetadata,
    pub path: String,
    archive: RefCell<ZipArchive<File>>, //This probably should be moved to a separate struct
    spine: BookSpine,
    pub manifest: BookManifest,
    pub table_of_contents: TableOfContents,
    _content_dir: PathBuf,
    // pub reader_current_item: Option<TableOfContentsItem>,
}

/// The path to the container.xml file in the META-INF directory
/// This file contains the path to the OPF file
/// Based on the epub 3.3 standard
/// https://www.w3.org/TR/epub-33/#sec-parsing-urls-metainf
/// 4.2.6.3.1
pub const META_INF_CONTAINER_PATH: &str = "META-INF/container.xml";

impl EBook {
    pub fn read_epub(epub_path: String) -> Result<EBook, Box<dyn std::error::Error>> {
        let epub_file = File::open(epub_path.clone())?;
        let mut archive = ZipArchive::new(epub_file)?;

        let opf_path = EBook::parse_container(&mut archive)?;
        let book = EBook::parse_opf(archive, &opf_path, epub_path)?;

        Ok(book)
    }

    fn parse_container(zip: &mut ZipArchive<File>) -> Result<String, Box<dyn std::error::Error>> {
        //TODO: Check whether that should be dynamic of is it a standard for EPUBs
        let mut container_file = zip.by_name(META_INF_CONTAINER_PATH)?;
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
            Err("OPF file not found".into())
        } else {
            Ok(opf_path)
        }
    }

    /// Assume `opf_path` is the path obtained from the previous step
    fn parse_opf(
        mut zip: ZipArchive<File>,
        opf_path: &str,
        epub_path: String,
    ) -> Result<EBook, Box<dyn std::error::Error>> {
        let opf_content = EBook::get_archive_file_content(zip.borrow_mut(), opf_path)
            .unwrap_or_else(|err| {
                eprintln!("{:?}", err);
                "NONE".to_string()
            });

        //TODO: Looks really junky to do it like this, potential for improvement in getting content_dir path
        //If OPS directory is a common thing for all books it should be hardcoded, if not, then some better way for getting root directory for book resources will be needed
        let content_dir = std::path::Path::new(opf_path).parent().unwrap().to_owned();

        let (manifest, spine, metadata) = EBook::create_from_opf(&opf_content);

        let table_of_contents = TableOfContents::create_table_of_contents(
            zip.borrow_mut(),
            &manifest,
            content_dir.borrow(),
        );

        Ok(Self {
            manifest,
            spine,
            metadata,
            path: epub_path,
            table_of_contents,
            _content_dir: content_dir,
            archive: RefCell::new(zip),
        })
    }

    pub(crate) fn get_archive_file_content(
        zip: &mut ZipArchive<File>,
        resource_path: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut resource_file = zip.by_name(resource_path)?;
        let mut contents = String::new();

        resource_file.read_to_string(&mut contents)?;

        Ok(contents)
    }

    fn create_from_opf(opf_content: &String) -> (BookManifest, BookSpine, BookMetadata) {
        let manifest = BookManifest::from_opf(opf_content);
        let metadata = BookMetadata::from_opf(opf_content);
        let spine = BookSpine::from_opf_and_manifest(opf_content, manifest.borrow());

        (manifest, spine, metadata)
    }

    pub fn get_content_by_toc_item(
        &mut self,
        toc_item: &TableOfContentsItem,
    ) -> Result<String, Box<dyn std::error::Error>> {
        //TODO: Fix it, so that subdirectories of epub file are detected automatically

        let mut archive = self.archive.borrow_mut();
        let mut target_file_content = archive.by_name(&toc_item.path)?;
        let mut contents = String::new();

        target_file_content.read_to_string(&mut contents)?;

        Ok(contents)
    }
}

#[cfg(test)]
mod book_tests {
    use crate::manifest::ManifestItem;

    use super::*;

    //TODO: Consider changing that test
    #[test]
    fn parse_container_should_return_path_to_opf() {
        let epub_file = File::open("./data/moby-dick.epub").unwrap();
        let mut archive = ZipArchive::new(epub_file).unwrap();

        let opf_path = EBook::parse_container(&mut archive);

        assert!(opf_path.is_ok());
        assert_eq!(opf_path.unwrap(), "OPS/package.opf")
    }

    #[test]
    fn read_epub_should_detect_content_directory() {
        let book = EBook::read_epub("./data/moby-dick.epub".to_string()).unwrap();

        assert_eq!(book._content_dir.to_str().unwrap(), "OPS");
    }

    #[test]
    fn parse_opf_should_return_book_with_correct_metadata() {
        let book = EBook::read_epub("./data/moby-dick.epub".to_string()).unwrap();

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

        let book = EBook::read_epub("./data/moby-dick.epub".to_string()).unwrap();

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
        let book = EBook::read_epub("./data/moby-dick.epub".to_string()).unwrap();

        let manifest = book.manifest;

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
