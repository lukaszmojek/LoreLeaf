use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs::File;
use std::io::Read;
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
    // let mut buf = Vec::new();

    // Find the OPF file path in the container XML
    let mut opf_path = String::new();
    // loop {
    //     match reader.read_event() {
    //         Event::Start(ref e) if e.name() == b"rootfile" => {
    //             for attribute in e.attributes() {
    //                 let attr = attribute?;
    //                 if attr.key == b"full-path" {
    //                     opf_path = String::from_utf8(attr.value.into_owned())?;
    //                     break;
    //                 }
    //             }
    //         }
    //         Event::Eof => break,
    //         _ => (),
    //     }
    //     buf.clear();
    // }

    if opf_path.is_empty() {
        Err("OPF file not found".into())
    } else {
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
    fn it_works() {
        let _ = read_epub();
    }
}
