mod book_reading_tests {
    use book_analyzer::{BookDetails, read_specific_book};

    #[test]
    fn should_return_some_with_details_when_book_exists() {
        let book_path = "./tests/data/1_Adept.epub".to_string();
        let expected_details = BookDetails { title: "Adept. Część I".to_string(), author: "Przechrzta Adam".to_string() };

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
