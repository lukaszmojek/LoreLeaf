mod book_reading_tests {
    use rstest::rstest;
    use book_analyzer::{BookDetails, read_specific_book};

    //DISCLAIMER: those books are free and downloaded from https://www.gutenberg.org/
    #[rstest]
    #[case::the_wonderful_wizard_of_oz("the_wonderful_wizard_of_oz", "The Wonderful Wizard of Oz", "L. Frank Baum")]
    #[case::the_time_machine("the_time_machine", "The Time Machine", "H. G. Wells")]
    fn should_return_some_with_details_when_book_exists(#[case] file_name: String, #[case] title: String, #[case] author: String) {
        let book_path = format!("./tests/data/{file_name}.epub");
        let expected_details = BookDetails { title, author };

        let option_result = read_specific_book(book_path);

        assert!(option_result.is_some());
        assert_eq!(option_result.unwrap(), expected_details);
    }

    #[test]
    fn should_return_none_when_book_not_exists() {
        let book_path = "./tests/data/not_existing.epub".to_string();

        let option_result = read_specific_book(book_path);

        assert!(option_result.is_none());
    }
}
