use std::{borrow::Borrow, collections::LinkedList};

use quick_xml::{events::Event, name::QName, Reader};

#[derive(Clone)]
pub struct TableOfContents {
    // pub items: LinkedList<TableOfContentsItem>,
    pub items: Vec<TableOfContentsItem>,
    // items_iter: Iter<TableOfContentsItem>,
    // pub current_item
}

#[derive(Debug, Clone)]
pub struct TableOfContentsItem {
    // id: String,
    pub path: String,
    pub label: String,
    pub content: Option<String>,
}

impl TableOfContentsItem {
    pub fn get_href_attribute(e: &quick_xml::events::BytesStart<'_>) -> String {
        let mut href: String = "".to_string();

        for attribute in e.attributes() {
            let attr = attribute.unwrap();

            if attr.key == QName(b"href") {
                href = String::from_utf8(attr.value.to_vec()).unwrap();
            }
        }

        href
    }
}

impl TableOfContents {
    pub fn from_content(toc_content: String) -> TableOfContents {
        let mut reader = Reader::from_str(toc_content.borrow());
        reader.trim_text(true);

        let navigation_selector: String = "toc".to_string();

        let mut buf = Vec::new();
        let mut toc_items: Vec<TableOfContentsItem> = vec![];

        let mut toc_item_href: String = "".to_string();
        let mut toc_item_label: String = "".to_string();
        let mut toc_item_reading_started: bool = false;
        let mut is_inside_toc_nav: bool = false;

        while let Ok(event) = reader.read_event_into(&mut buf) {
            match event {
                Event::Start(ref e) | Event::Empty(ref e) => {
                    //TODO: Refactor that attribute check
                    _ = e.attributes().any(|x| -> bool {
                        if let Ok(attr) = x {
                            if attr.key == QName(b"epub:type") {
                                let ns = String::from_utf8(attr.value.to_vec()).unwrap();
                                is_inside_toc_nav = ns == navigation_selector;
                            }
                        }

                        false
                    });

                    if let b"a" = e.name().as_ref() {
                        toc_item_href = TableOfContentsItem::get_href_attribute(e);
                        toc_item_reading_started = true;
                    }
                }
                Event::Text(e) => {
                    toc_item_label = e.unescape().unwrap().to_string();
                }
                Event::End(e) => {
                    if !toc_item_reading_started {
                        continue;
                    }

                    if !is_inside_toc_nav {
                        break;
                    }

                    let toc_item = TableOfContentsItem {
                        path: toc_item_href,
                        label: toc_item_label,
                        content: None,
                    };

                    toc_items.push(toc_item);
                    toc_item_href = "".to_string();
                    toc_item_label = "".to_string();
                    toc_item_reading_started = false;
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }

        TableOfContents { items: toc_items }
    }
}

#[cfg(test)]
mod table_of_contents_tests {
    use crate::EBook;

    use super::*;

    #[test]
    fn should_contain_properly_read_items_of_the_book() {
        let book = EBook::read_epub("./data/moby-dick.epub".to_string()).unwrap();

        let table_of_contents = book.table_of_contents;

        let toc_length = table_of_contents.items.len();

        assert_eq!(toc_length, 141);

        assert_eq!(table_of_contents.items[0].path, "titlepage.xhtml");
        assert_eq!(table_of_contents.items[0].label, "Moby-Dick");

        assert_eq!(
            table_of_contents.items[toc_length - 3].path,
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

        assert_eq!(selected_toc_item.path, "chapter_135.xhtml");
        assert_eq!(
            selected_toc_item.label,
            "Chapter 135. The Chase.—Third Day."
        );
        //Adding all characters count and the new line characters which are not displayed
        assert_eq!(toc_item_content.len(), 26305 + 73);
    }

    #[test]
    fn get_first_toc_item() {
        let content = r#"<?xml version="1.0" encoding="UTF-8"?>
        <html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops">
           <head>
              <title>Moby-Dick</title>
              <link rel="stylesheet" href="css/stylesheet.css" type="text/css"/>
              <meta charset="utf-8"/>
           </head>
           <body>
              <section class="frontmatter TableOfContents" epub:type="frontmatter toc">
                 <header>
                    <h1>Contents</h1>
                 </header>
                 <nav xmlns:epub="http://www.idpf.org/2007/ops" epub:type="toc" id="toc">
                    <ol>
                       <li class="toc-BookTitlePage-rw" id="toc-titlepage">
                          <a href="titlepage.xhtml">Moby-Dick</a>
                       </li>
                       <li class="toc-Preface-rw" id="toc-preface_001">
                          <a href="preface_001.xhtml">Original Transcriber’s Notes:</a>
                       </li>
                       <li class="toc-Introduction-rw" id="toc-introduction_001">
                          <a href="introduction_001.xhtml">ETYMOLOGY.</a>
                       </li>
                       <li class="toc-Epigraph-rw" id="toc-epigraph_001">
                          <a href="epigraph_001.xhtml">EXTRACTS (Supplied by a Sub-Sub-Librarian).</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_001">
                          <a href="chapter_001.xhtml">Chapter 1. Loomings.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_002">
                          <a href="chapter_002.xhtml">Chapter 2. The Carpet-Bag.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_003">
                          <a href="chapter_003.xhtml">Chapter 3. The Spouter-Inn.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_004">
                          <a href="chapter_004.xhtml">Chapter 4. The Counterpane.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_005">
                          <a href="chapter_005.xhtml">Chapter 5. Breakfast.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_006">
                          <a href="chapter_006.xhtml">Chapter 6. The Street.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_007">
                          <a href="chapter_007.xhtml">Chapter 7. The Chapel.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_008">
                          <a href="chapter_008.xhtml">Chapter 8. The Pulpit.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_009">
                          <a href="chapter_009.xhtml">Chapter 9. The Sermon.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_010">
                          <a href="chapter_010.xhtml">Chapter 10. A Bosom Friend.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_011">
                          <a href="chapter_011.xhtml">Chapter 11. Nightgown.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_012">
                          <a href="chapter_012.xhtml">Chapter 12. Biographical.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_013">
                          <a href="chapter_013.xhtml">Chapter 13. Wheelbarrow.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_014">
                          <a href="chapter_014.xhtml">Chapter 14. Nantucket.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_015">
                          <a href="chapter_015.xhtml">Chapter 15. Chowder.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_016">
                          <a href="chapter_016.xhtml">Chapter 16. The Ship.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_017">
                          <a href="chapter_017.xhtml">Chapter 17. The Ramadan.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_018">
                          <a href="chapter_018.xhtml">Chapter 18. His Mark.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_019">
                          <a href="chapter_019.xhtml">Chapter 19. The Prophet.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_020">
                          <a href="chapter_020.xhtml">Chapter 20. All Astir.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_021">
                          <a href="chapter_021.xhtml">Chapter 21. Going Aboard.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_022">
                          <a href="chapter_022.xhtml">Chapter 22. Merry Christmas.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_023">
                          <a href="chapter_023.xhtml">Chapter 23. The Lee Shore.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_024">
                          <a href="chapter_024.xhtml">Chapter 24. The Advocate.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_025">
                          <a href="chapter_025.xhtml">Chapter 25. Postscript.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_026">
                          <a href="chapter_026.xhtml">Chapter 26. Knights and Squires.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_027">
                          <a href="chapter_027.xhtml">Chapter 27. Knights and Squires.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_028">
                          <a href="chapter_028.xhtml">Chapter 28. Ahab.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_029">
                          <a href="chapter_029.xhtml">Chapter 29. Enter Ahab; to Him, Stubb.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_030">
                          <a href="chapter_030.xhtml">Chapter 30. The Pipe.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_031">
                          <a href="chapter_031.xhtml">Chapter 31. Queen Mab.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_032">
                          <a href="chapter_032.xhtml">Chapter 32. Cetology.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_033">
                          <a href="chapter_033.xhtml">Chapter 33. The Specksnyder.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_034">
                          <a href="chapter_034.xhtml">Chapter 34. The Cabin-Table.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_035">
                          <a href="chapter_035.xhtml">Chapter 35. The Mast-Head.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_036">
                          <a href="chapter_036.xhtml">Chapter 36. The Quarter-Deck.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_037">
                          <a href="chapter_037.xhtml">Chapter 37. Sunset.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_038">
                          <a href="chapter_038.xhtml">Chapter 38. Dusk.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_039">
                          <a href="chapter_039.xhtml">Chapter 39. First Night Watch.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_040">
                          <a href="chapter_040.xhtml">Chapter 40. Midnight, Forecastle.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_041">
                          <a href="chapter_041.xhtml">Chapter 41. Moby Dick.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_042">
                          <a href="chapter_042.xhtml">Chapter 42. The Whiteness of The Whale.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_043">
                          <a href="chapter_043.xhtml">Chapter 43. Hark!</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_044">
                          <a href="chapter_044.xhtml">Chapter 44. The Chart.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_045">
                          <a href="chapter_045.xhtml">Chapter 45. The Affidavit.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_046">
                          <a href="chapter_046.xhtml">Chapter 46. Surmises.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_047">
                          <a href="chapter_047.xhtml">Chapter 47. The Mat-Maker.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_048">
                          <a href="chapter_048.xhtml">Chapter 48. The First Lowering.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_049">
                          <a href="chapter_049.xhtml">Chapter 49. The Hyena.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_050">
                          <a href="chapter_050.xhtml">Chapter 50. Ahab’s Boat and Crew. Fedallah.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_051">
                          <a href="chapter_051.xhtml">Chapter 51. The Spirit-Spout.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_052">
                          <a href="chapter_052.xhtml">Chapter 52. The Albatross.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_053">
                          <a href="chapter_053.xhtml">Chapter 53. The Gam.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_054">
                          <a href="chapter_054.xhtml">Chapter 54. The Town-Ho’s Story.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_055">
                          <a href="chapter_055.xhtml">Chapter 55. Of the Monstrous Pictures of Whales.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_056">
                          <a href="chapter_056.xhtml">Chapter 56. Of the Less Erroneous Pictures of Whales, and the True</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_057">
                          <a href="chapter_057.xhtml">Chapter 57. Of Whales in Paint; in Teeth; in Wood; in Sheet-Iron; in</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_058">
                          <a href="chapter_058.xhtml">Chapter 58. Brit.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_059">
                          <a href="chapter_059.xhtml">Chapter 59. Squid.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_060">
                          <a href="chapter_060.xhtml">Chapter 60. The Line.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_061">
                          <a href="chapter_061.xhtml">Chapter 61. Stubb Kills a Whale.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_062">
                          <a href="chapter_062.xhtml">Chapter 62. The Dart.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_063">
                          <a href="chapter_063.xhtml">Chapter 63. The Crotch.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_064">
                          <a href="chapter_064.xhtml">Chapter 64. Stubb’s Supper.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_065">
                          <a href="chapter_065.xhtml">Chapter 65. The Whale as a Dish.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_066">
                          <a href="chapter_066.xhtml">Chapter 66. The Shark Massacre.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_067">
                          <a href="chapter_067.xhtml">Chapter 67. Cutting In.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_068">
                          <a href="chapter_068.xhtml">Chapter 68. The Blanket.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_069">
                          <a href="chapter_069.xhtml">Chapter 69. The Funeral.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_070">
                          <a href="chapter_070.xhtml">Chapter 70. The Sphynx.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_071">
                          <a href="chapter_071.xhtml">Chapter 71. The Jeroboam’s Story.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_072">
                          <a href="chapter_072.xhtml">Chapter 72. The Monkey-Rope.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_073">
                          <a href="chapter_073.xhtml">Chapter 73. Stubb and Flask Kill a Right Whale; and Then Have a Talk</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_074">
                          <a href="chapter_074.xhtml">Chapter 74. The Sperm Whale’s Head—Contrasted View.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_075">
                          <a href="chapter_075.xhtml">Chapter 75. The Right Whale’s Head—Contrasted View.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_076">
                          <a href="chapter_076.xhtml">Chapter 76. The Battering-Ram.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_077">
                          <a href="chapter_077.xhtml">Chapter 77. The Great Heidelburgh Tun.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_078">
                          <a href="chapter_078.xhtml">Chapter 78. Cistern and Buckets.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_079">
                          <a href="chapter_079.xhtml">Chapter 79. The Prairie.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_080">
                          <a href="chapter_080.xhtml">Chapter 80. The Nut.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_081">
                          <a href="chapter_081.xhtml">Chapter 81. The Pequod Meets The Virgin.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_082">
                          <a href="chapter_082.xhtml">Chapter 82. The Honour and Glory of Whaling.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_083">
                          <a href="chapter_083.xhtml">Chapter 83. Jonah Historically Regarded.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_084">
                          <a href="chapter_084.xhtml">Chapter 84. Pitchpoling.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_085">
                          <a href="chapter_085.xhtml">Chapter 85. The Fountain.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_086">
                          <a href="chapter_086.xhtml">Chapter 86. The Tail.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_087">
                          <a href="chapter_087.xhtml">Chapter 87. The Grand Armada.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_088">
                          <a href="chapter_088.xhtml">Chapter 88. Schools and Schoolmasters.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_089">
                          <a href="chapter_089.xhtml">Chapter 89. Fast-Fish and Loose-Fish.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_090">
                          <a href="chapter_090.xhtml">Chapter 90. Heads or Tails.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_091">
                          <a href="chapter_091.xhtml">Chapter 91. The Pequod Meets The Rose-Bud.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_092">
                          <a href="chapter_092.xhtml">Chapter 92. Ambergris.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_093">
                          <a href="chapter_093.xhtml">Chapter 93. The Castaway.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_094">
                          <a href="chapter_094.xhtml">Chapter 94. A Squeeze of the Hand.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_095">
                          <a href="chapter_095.xhtml">Chapter 95. The Cassock.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_096">
                          <a href="chapter_096.xhtml">Chapter 96. The Try-Works.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_097">
                          <a href="chapter_097.xhtml">Chapter 97. The Lamp.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_098">
                          <a href="chapter_098.xhtml">Chapter 98. Stowing Down and Clearing Up.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_099">
                          <a href="chapter_099.xhtml">Chapter 99. The Doubloon.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_100">
                          <a href="chapter_100.xhtml">Chapter 100. Leg and Arm.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_101">
                          <a href="chapter_101.xhtml">Chapter 101. The Decanter.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_102">
                          <a href="chapter_102.xhtml">Chapter 102. A Bower in the Arsacides.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_103">
                          <a href="chapter_103.xhtml">Chapter 103. Measurement of The Whale’s Skeleton.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_104">
                          <a href="chapter_104.xhtml">Chapter 104. The Fossil Whale.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_105">
                          <a href="chapter_105.xhtml">Chapter 105. Does the Whale’s Magnitude Diminish?—Will He Perish?</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_106">
                          <a href="chapter_106.xhtml">Chapter 106. Ahab’s Leg.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_107">
                          <a href="chapter_107.xhtml">Chapter 107. The Carpenter.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_108">
                          <a href="chapter_108.xhtml">Chapter 108. Ahab and the Carpenter.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_109">
                          <a href="chapter_109.xhtml">Chapter 109. Ahab and Starbuck in the Cabin.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_110">
                          <a href="chapter_110.xhtml">Chapter 110. Queequeg in His Coffin.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_111">
                          <a href="chapter_111.xhtml">Chapter 111. The Pacific.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_112">
                          <a href="chapter_112.xhtml">Chapter 112. The Blacksmith.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_113">
                          <a href="chapter_113.xhtml">Chapter 113. The Forge.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_114">
                          <a href="chapter_114.xhtml">Chapter 114. The Gilder.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_115">
                          <a href="chapter_115.xhtml">Chapter 115. The Pequod Meets The Bachelor.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_116">
                          <a href="chapter_116.xhtml">Chapter 116. The Dying Whale.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_117">
                          <a href="chapter_117.xhtml">Chapter 117. The Whale Watch.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_118">
                          <a href="chapter_118.xhtml">Chapter 118. The Quadrant.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_119">
                          <a href="chapter_119.xhtml">Chapter 119. The Candles.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_120">
                          <a href="chapter_120.xhtml">Chapter 120. The Deck Towards the End of the First Night Watch.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_121">
                          <a href="chapter_121.xhtml">Chapter 121. Midnight.—The Forecastle Bulwarks.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_122">
                          <a href="chapter_122.xhtml">Chapter 122. Midnight Aloft.—Thunder and Lightning.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_123">
                          <a href="chapter_123.xhtml">Chapter 123. The Musket.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_124">
                          <a href="chapter_124.xhtml">Chapter 124. The Needle.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_125">
                          <a href="chapter_125.xhtml">Chapter 125. The Log and Line.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_126">
                          <a href="chapter_126.xhtml">Chapter 126. The Life-Buoy.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_127">
                          <a href="chapter_127.xhtml">Chapter 127. The Deck.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_128">
                          <a href="chapter_128.xhtml">Chapter 128. The Pequod Meets The Rachel.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_129">
                          <a href="chapter_129.xhtml">Chapter 129. The Cabin.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_130">
                          <a href="chapter_130.xhtml">Chapter 130. The Hat.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_131">
                          <a href="chapter_131.xhtml">Chapter 131. The Pequod Meets The Delight.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_132">
                          <a href="chapter_132.xhtml">Chapter 132. The Symphony.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_133">
                          <a href="chapter_133.xhtml">Chapter 133. The Chase—First Day.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_134">
                          <a href="chapter_134.xhtml">Chapter 134. The Chase—Second Day.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_135">
                          <a href="chapter_135.xhtml">Chapter 135. The Chase.—Third Day.</a>
                       </li>
                       <li class="toc-Chapter-rw" id="toc-chapter_136">
                          <a href="chapter_136.xhtml">Epilogue</a>
                       </li>
                       <li>
                          <a href="copyright.xhtml">Copyright Page</a>
                       </li>
                    </ol>
                 </nav>
              </section>
           </body>
        </html>
"#;

        let sut = TableOfContents::from_content(content.to_string());

        let toc_iter = sut.items.iter();
        // assert_eq!("s");
    }
}
