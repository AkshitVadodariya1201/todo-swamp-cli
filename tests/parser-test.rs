#[cfg(test)]
mod tests {

    use todo_swamp::parser::SearchWordOrTag;
    use todo_swamp::parser::*;
    use todo_swamp::*;

    #[test]
    fn test_query() {
        assert!(query("add \"description\" #tag").is_ok());
        assert!(query("done 123").is_ok());
        assert!(query("search term #tag").is_ok());
    }

    #[test]
    fn test_ws() {
        assert_eq!(ws(" "), Ok(("", ' ')));
        assert_eq!(ws("\t"), Ok(("", '\t')));
        assert!(ws("a").is_err());
    }

    #[test]
    fn test_is_lowecase_or_dash_or_whitespace() {
        assert!(is_lowecase_or_dash_or_whitespace('a'));
        assert!(is_lowecase_or_dash_or_whitespace('-'));
        assert!(is_lowecase_or_dash_or_whitespace(' '));
        assert!(!is_lowecase_or_dash_or_whitespace('A'));
    }

    #[test]
    fn test_is_lowecase_or_dash() {
        assert!(is_lowecase_or_dash('a'));
        assert!(is_lowecase_or_dash('-'));
        assert!(!is_lowecase_or_dash(' '));
        assert!(!is_lowecase_or_dash('A'));
    }

    #[test]
    fn test_sentence() {
        assert_eq!(
            sentence("this is a sentence"),
            Ok(("", "this is a sentence"))
        );
    }

    #[test]
    fn test_word() {
        assert_eq!(word("word"), Ok(("", "word")));
        assert!(word("Word").is_err());
    }

    #[test]
    fn test_todo_tag() {
        assert_eq!(todo_tag("#tag"), Ok(("", "tag")));
        assert!(todo_tag("tag").is_err());
    }

    #[test]
    fn test_description() {
        assert_eq!(
            description("\"description\""),
            Ok(("", "description".to_string()))
        );
        assert!(description("description").is_err());
    }

    #[test]
    fn test_done() {
        assert_eq!(done("done 123"), Ok(("", Query::Done(Index::new(123)))));
        assert!(done("done abc").is_err());
    }

    #[test]
    fn test_vec_to_u64() {
        assert_eq!(vec_to_u64(vec!["1", "2", "3"]), 123);
        assert_eq!(vec_to_u64(vec!["12", "34"]), 1234);
    }

    #[test]
    fn test_search_valid() {
        let input = "search word1";
        let result = search(input);
        assert!(result.is_ok());
        let (rest, query) = result.unwrap();
        assert_eq!(rest, "1");
        if let Query::Search(params) = query {
            assert_eq!(params.words.len(), 1);
            assert_eq!(params.tags.len(), 0);
        } else {
            panic!("Expected Query::Search");
        }
    }

    #[test]
    fn test_search_word_or_tag() {
        assert_eq!(
            search_word_or_tag("#tag"),
            Ok(("", SearchWordOrTag::RawTag("tag".to_string())))
        );
        assert_eq!(
            search_word_or_tag("word"),
            Ok(("", SearchWordOrTag::RawWord("word".to_string())))
        );
        assert!(search_word_or_tag("123").is_err());
    }

    #[test]
    fn test_mash_to_query() {
        let mash = vec![
            SearchWordOrTag::RawWord("word".to_string()),
            SearchWordOrTag::RawTag("tag".to_string()),
        ];
        let query = mash_to_query(mash);
        if let Query::Search(params) = query {
            assert_eq!(params.words.len(), 1);
            assert_eq!(params.tags.len(), 1);
        } else {
            panic!("Expected Query::Search");
        }
    }
}
