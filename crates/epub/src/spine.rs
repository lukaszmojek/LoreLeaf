use std::{rc::Rc, sync::Arc};

use quick_xml::{events::Event, name::QName, Reader};

use crate::{BookManifest, ManifestItem};

#[derive(Debug)]
pub struct BookSpine {
    pub items: Vec<BookSpineItem>,
}

#[derive(Debug)]
pub struct BookSpineItem {
    pub id: String,
    pub value: Arc<ManifestItem>,
}

impl BookSpine {
    pub fn from_opf_and_manifest(opf_content: &str, manifest: &BookManifest) -> BookSpine {
        let mut reader = Reader::from_str(opf_content);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut spine: Vec<BookSpineItem> = Vec::new();

        while let Ok(event) = reader.read_event_into(&mut buf) {
            match event {
                Event::Start(ref e) | Event::Empty(ref e) => {
                    // current_tag = String::from_utf8(e.name().as_ref().to_vec()).unwrap();
                    if let b"itemref" = e.name().as_ref() {
                        BookSpine::recreate_spine_item(e, &mut spine, manifest);
                    }
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }

        BookSpine { items: spine }
    }

    fn recreate_spine_item(
        e: &quick_xml::events::BytesStart<'_>,
        spine: &mut Vec<BookSpineItem>,
        manifest: &BookManifest,
    ) {
        for attribute in e.attributes() {
            let attr = attribute.unwrap();
            if attr.key == QName(b"idref") {
                let item_id = String::from_utf8(attr.value.into_owned()).unwrap();
                let item = manifest.items.iter().find(|i| i.id == item_id).unwrap();

                let spine_item = BookSpineItem {
                    id: item_id,
                    value: Arc::clone(item),
                };
                spine.push(spine_item);
                break;
            }
        }
    }
}
