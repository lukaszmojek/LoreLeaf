mod manifest;
mod metadata;
mod spine;
mod table_of_contents;

use manifest::{BookManifest, ManifestItem};
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;
use spine::BookSpine;
use std::borrow::{Borrow, BorrowMut};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use table_of_contents::{TableOfContents, TableOfContentsItem};
use zip::ZipArchive;

use crate::metadata::BookMetadata;

pub struct EBook {
    pub metadata: BookMetadata,
    pub path: String,
    archive: ZipArchive<File>,
    spine: BookSpine,
    manifest: BookManifest,
    table_of_contents: TableOfContents,
    _content_dir: PathBuf,
    // pub reader_current_item: Option<TableOfContentsItem>,
}

impl EBook {
    pub fn read_epub(epub_path: String) -> Result<(EBook), Box<dyn std::error::Error>> {
        let epub_file = File::open(epub_path.clone())?;
        let mut archive = ZipArchive::new(epub_file)?;

        let opf_path = EBook::parse_container(&mut archive)?;
        let book = EBook::parse_opf(archive, &opf_path, epub_path)?;

        Ok(book)
    }

    fn parse_container(zip: &mut ZipArchive<File>) -> Result<String, Box<dyn std::error::Error>> {
        //TODO: Check whether that should be dynamic of is it a standard for EPUBs
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

        let content_dir = std::path::Path::new(opf_path).parent().unwrap().to_owned();

        let (manifest, spine, metadata) = EBook::create_from_opf(&opf_content);

        let table_of_contents =
            EBook::create_table_of_contents(zip.borrow_mut(), &manifest, content_dir.borrow());

        Ok(Self {
            manifest,
            spine,
            metadata,
            path: epub_path,
            table_of_contents,
            _content_dir: content_dir,
            archive: zip,
        })
    }

    fn create_table_of_contents(
        zip: &mut ZipArchive<File>,
        manifest: &BookManifest,
        content_dir: &Path,
    ) -> TableOfContents {
        // todo!("Href here is not absolute, it is relative to opf file. This needs to be addressed.");
        let table_of_contents_from_manifest = manifest.search_for_item("toc").unwrap();

        let toc_path = content_dir.join(table_of_contents_from_manifest.href.clone());

        let toc_content = EBook::get_archive_file_content(zip, toc_path.to_str().unwrap())
            .unwrap_or_else(|err| {
                eprintln!("{:?}", err);
                "NONE".to_string()
            });

        TableOfContents::from_content(toc_content)
    }

    fn get_archive_file_content(
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

    fn get_content_by_toc_item(
        &mut self,
        toc_item: &TableOfContentsItem,
    ) -> Result<String, Box<dyn std::error::Error>> {
        //TODO: Fix it, so that subdirectories of epub file are detected automatically

        let relative_path = self.get_resource_path(&toc_item.href);

        let mut opf_file = self.archive.by_name(&relative_path)?;
        let mut contents = String::new();

        opf_file.read_to_string(&mut contents)?;

        Ok(contents)
    }

    fn get_resource_path(&mut self, item_href: &str) -> String {
        let relative_path = self._content_dir.to_str().unwrap_or("").to_string() + "/" + item_href;

        relative_path
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

#[cfg(test)]
mod manifest_tests {
    use super::*;

    #[test]
    fn search_for_item_should_return_matching_item_when_queried() {
        let book = EBook::read_epub("./data/moby-dick.epub".to_string()).unwrap();

        let manifest = book.manifest;

        let toc_from_manifest = manifest
            .search_for_item("toc")
            .unwrap_or_else(|| panic!("toc was not found during search in manifest"));

        assert_eq!(toc_from_manifest.id, "toc");
        assert_eq!(toc_from_manifest.href, "toc.xhtml");
        assert_eq!(toc_from_manifest.media_type, "application/xhtml+xml");
    }
}

#[cfg(test)]
mod table_of_contents_tests {
    use super::*;

    #[test]
    fn should_contain_properly_read_items_of_the_book() {
        let book = EBook::read_epub("./data/moby-dick.epub".to_string()).unwrap();

        let table_of_contents = book.table_of_contents;

        let toc_length = table_of_contents.items.len();

        assert_eq!(toc_length, 141);

        assert_eq!(table_of_contents.items[0].href, "titlepage.xhtml");
        assert_eq!(table_of_contents.items[0].label, "Moby-Dick");

        assert_eq!(
            table_of_contents.items[toc_length - 3].href,
            "chapter_135.xhtml"
        );
        assert_eq!(
            table_of_contents.items[toc_length - 3].label,
            "Chapter 135. The Chase.—Third Day."
        );
    }

    #[test]
    fn reader_should_get_the_content_based_on_toc_item() {
        let mut book = EBook::read_epub("./data/moby-dick.epub".to_string()).unwrap();

        let table_of_contents = book.table_of_contents.clone();

        let toc_length = table_of_contents.items.len();
        let selected_toc_item = table_of_contents.items[toc_length - 3].clone();

        let toc_item_content = book.get_content_by_toc_item(&selected_toc_item).unwrap();

        assert_eq!(selected_toc_item.href, "chapter_135.xhtml");
        assert_eq!(
            selected_toc_item.label,
            "Chapter 135. The Chase.—Third Day."
        );
        //Adding all characters count and the new line characters which are not displayed
        assert_eq!(toc_item_content.len(), 26305 + 73);
    }
}
