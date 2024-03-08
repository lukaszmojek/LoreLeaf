use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs::File;
use std::io::Read;
use quick_xml::name::QName;
use zip::ZipArchive;

fn read_epub() -> Result<(), Box<dyn std::error::Error>> {
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
                match e.name().as_ref() {
                    b"rootfile" => {
                        for attribute in e.attributes() {
                            let attr = attribute?;
                            if attr.key == QName(b"full-path") {
                                opf_path = String::from_utf8(attr.value.into_owned())?;
                                break;
                            }
                        }
                    },
                    _ => (),
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
fn parse_opf(zip: &mut ZipArchive<File>, opf_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut opf_file = zip.by_name(opf_path)?;
    let mut contents = String::new();
    opf_file.read_to_string(&mut contents)?;

    // Use quick-xml or another XML parser to parse the contents
    // Extract metadata, manifest items, and spine order

    Ok(())
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
}
