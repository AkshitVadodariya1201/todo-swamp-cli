// Test functions
#[cfg(test)]
mod tests {

    use todo_swamp::parser::query;
    use todo_swamp::*;

    #[test]
    fn test_standard_generator_add() {
        let input = r#"add "example description" #tag1 #tag2"#;
        let result = query(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_standard_generator_search() {
        let input = r#"search example #tag1"#;
        let result = query(input);
        assert!(matches!(result, Ok((_, Query::Search(..)))));
    }

    #[test]
    fn test_standard_generator_done() {
        let input = r#"done 1"#;
        let result = query(input);
        assert!(matches!(result, Ok((_, Query::Done(..)))));
    }

    #[test]
    fn test_standard_generator_mixed() {
        let input = r#"add "example description" #tag1 #tag2"#;
        let result = query(input);
        assert!(result.is_ok());

        let input = r#"search example #tag1"#;
        let result = query(input);
        assert!(matches!(result, Ok((_, Query::Search(..)))));

        let input = r#"done 1"#;
        let result = query(input);
        assert!(matches!(result, Ok((_, Query::Done(..)))));
    }

    #[test]
    fn test_ratiodone_generator_add_done() {
        let input = r#"add "example description" #tag1 #tag2"#;
        let result = query(input);
        assert!(result.is_ok());

        let input = r#"done 1"#;
        let result = query(input);
        assert!(matches!(result, Ok((_, Query::Done(..)))));
    }

    #[test]
    fn test_ratiodone_generator_add_done_search() {
        let input = r#"add "example description" #tag1 #tag2"#;
        let result = query(input);
        assert!(result.is_ok());

        let input = r#"done 1"#;
        let result = query(input);
        assert!(matches!(result, Ok((_, Query::Done(..)))));

        let input = r#"search example #tag1"#;
        let result = query(input);
        assert!(matches!(result, Ok((_, Query::Search(..)))));
    }

    #[test]
    fn test_interleaved_generator_add_done() {
        let input = r#"add "example description" #tag1 #tag2"#;
        let result = query(input);
        assert!(result.is_ok());

        let input = r#"done 1"#;
        let result = query(input);
        assert!(matches!(result, Ok((_, Query::Done(..)))));

        let input = r#"add "another description" #tag3"#;
        let result = query(input);
        assert!(result.is_ok());

        let input = r#"done 2"#;
        let result = query(input);
        assert!(matches!(result, Ok((_, Query::Done(..)))));
    }

    #[test]
    fn test_interleaved_generator_add_done_search() {
        let input = r#"add "example description" #tag1 #tag2"#;
        let result = query(input);
        assert!(result.is_ok());

        let input = r#"done 1"#;
        let result = query(input);
        assert!(matches!(result, Ok((_, Query::Done(..)))));

        let input = r#"search example #tag1"#;
        let result = query(input);
        assert!(matches!(result, Ok((_, Query::Search(..)))));

        let input = r#"add "another description" #tag3"#;
        let result = query(input);
        assert!(result.is_ok());

        let input = r#"done 2"#;
        let result = query(input);
        assert!(matches!(result, Ok((_, Query::Done(..)))));
    }

    #[test]
    fn test_lattice_generator_add() {
        let input = r#"add "example description" #tag1 #tag2"#;
        let result = query(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_lattice_generator_search() {
        let input = r#"search example #tag1"#;
        let result = query(input);
        assert!(matches!(result, Ok((_, Query::Search(..)))));
    }

    #[test]
    fn test_lattice_generator_done() {
        let input = r#"done 1"#;
        let result = query(input);
        assert!(matches!(result, Ok((_, Query::Done(..)))));
    }
}
