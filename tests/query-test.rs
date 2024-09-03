#[cfg(test)]
mod tests {
    use todo_swamp::todo_list::{Description, Index, Tag, TodoItem};
    use todo_swamp::{
        is_subsequence, is_subsequence_in_any_word, QueryError, QueryResult, SearchParams,
        SearchWord,
    };

    #[test]
    fn test_is_subsequence_in_any_word() {
        assert!(is_subsequence_in_any_word(
            "test",
            "This is a test description"
        ));
        assert!(!is_subsequence_in_any_word(
            "missing",
            "This is a test description"
        ));
        assert!(is_subsequence_in_any_word(
            "desc",
            "This is a test description"
        ));
    }

    #[test]
    fn test_is_subsequence() {
        assert!(is_subsequence("test", "This is a test description"));
        assert!(!is_subsequence("missing", "This is a test description"));
        assert!(is_subsequence("desc", "description"));
    }

    #[test]
    fn test_search_params_matches() {
        let description = Description::new("This is a test description");
        let tags = vec![Tag::new("tag1"), Tag::new("tag2")];
        let todo_item = TodoItem::new(Index::new(1).into(), description, tags.clone(), false);

        let search_words = vec![SearchWord::new("test"), SearchWord::new("description")];
        let search_tags = vec![Tag::new("tag1")];
        let search_params = SearchParams {
            words: search_words,
            tags: search_tags.clone(),
        };

        assert!(search_params.matches(&todo_item));

        let search_words = vec![SearchWord::new("missing")];
        let search_params = SearchParams {
            words: search_words,
            tags: search_tags.clone(),
        };

        assert!(!search_params.matches(&todo_item));

        let search_tags = vec![Tag::new("missing")];
        let search_params = SearchParams {
            words: vec![SearchWord::new("test")],
            tags: search_tags,
        };

        assert!(!search_params.matches(&todo_item));
    }

    #[test]
    fn test_search_word_new() {
        let word = "example";
        let search_word = SearchWord::new(word);
        assert_eq!(search_word.0, word.to_owned());
    }

    #[test]
    fn test_query_result_fmt() {
        let description = Description::new("This is a test description");
        let tags = vec![Tag::new("tag1"), Tag::new("tag2")];
        let todo_item = TodoItem::new(
            Index::new(1).into(),
            description.clone(),
            tags.clone(),
            false,
        );

        let query_result = QueryResult::Added(todo_item.clone());
        assert_eq!(format!("{}", query_result), "1");

        let query_result = QueryResult::Done;
        assert_eq!(format!("{}", query_result), "done");

        let query_result = QueryResult::Found(vec![todo_item.clone()]);
        let expected_output = format!(
            "1 item(s) found\n1 \"{}\" {}",
            description.value(),
            tags.iter()
                .map(|tag| format!("#{}", tag.value()))
                .collect::<Vec<_>>()
                .join(" ")
        );
        assert_eq!(format!("{}", query_result), expected_output);
    }

    #[test]
    fn test_query_error_fmt() {
        let error_message = "An error occurred";
        let query_error = QueryError(error_message.to_owned());
        assert_eq!(
            format!("{}", query_error),
            format!(
                "An error occurred while processing the query: {}.",
                error_message
            )
        );
    }
}
