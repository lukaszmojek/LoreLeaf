use quick_xml::{events::Event, Reader};

#[derive(Debug)]
pub struct BookMetadata {
    pub title: Option<String>,
    pub creator: Option<String>,
    pub identifier: Option<String>,
    pub language: Option<String>,
    pub publisher: Option<String>,
    pub rights: Option<String>,
}

//TODO: Consider adding implementation of 'cleaning up' the metadata to remove characters such as '-' and '_' from raw metadata strings
//NOTE: It should be applicable only to certain metadata such as Creator and Title
impl BookMetadata {
    pub fn from_opf(opf_content: &String) -> BookMetadata {
        let mut reader = Reader::from_str(opf_content);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut metadata = BookMetadata {
            title: None,
            creator: None,
            identifier: None,
            language: None,
            publisher: None,
            rights: None,
        };

        let mut current_tag = String::new();

        while let Ok(event) = reader.read_event_into(&mut buf) {
            match event {
                Event::Start(ref e) | Event::Empty(ref e) => {
                    current_tag = String::from_utf8(e.name().as_ref().to_vec()).unwrap();
                }
                Event::Text(e) => {
                    let text = e.unescape().unwrap().to_string();
                    match current_tag.as_str() {
                        "dc:title" => metadata.title = Some(text),
                        "dc:creator" => metadata.creator = Some(text),
                        "dc:identifier" => metadata.identifier = Some(text),
                        "dc:language" => metadata.language = Some(text),
                        "dc:publisher" => metadata.publisher = Some(text),
                        "dc:rights" => metadata.rights = Some(text),
                        _ => {}
                    }
                }
                Event::End(_) => {
                    current_tag.clear();
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }

        metadata
    }
}
