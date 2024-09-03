#[cfg(test)]
mod tests {
    use todo_swamp::{
        runner::run_query_async,
        Description, Index, Query, QueryResult, SearchParams, Tag, TodoList,
    };
    use async_std::task;

    #[test]
    fn test_run_query_add_async() {
        task::block_on(async {
            let mut todo_list = TodoList::new();
            let query = Query::Add(Description::new("Test task"), vec![Tag::new("tag1")]);
            let result = run_query_async(query, &mut todo_list).await;
            assert!(result.is_ok());
            if let Ok(QueryResult::Added(_index)) = result {
                assert_eq!(
                    todo_list.items[0].description,
                    Description::new("Test task")
                );
            } else {
                panic!("Expected QueryResult::Added");
            }
        });
    }

    #[test]
    fn test_run_query_done_async() {
        task::block_on(async {
            let mut todo_list = TodoList::new();
            todo_list.add_item("Test task".to_string(), vec!["tag1".to_string()]);
            let query = Query::Done(Index::new(0));
            let result = run_query_async(query, &mut todo_list).await;
            assert!(result.is_ok());
            if let Ok(QueryResult::Done) = result {
                assert!(todo_list.items[0].done);
            } else {
                panic!("Expected QueryResult::Done");
            }
        });
    }

    #[test]
    fn test_run_query_done_invalid_index_async() {
        task::block_on(async {
            let mut todo_list = TodoList::new();
            let query = Query::Done(Index::new(0));
            let result = run_query_async(query, &mut todo_list).await;
            assert!(result.is_err());
        });
    }

    #[test]
    fn test_run_query_search_async() {
        task::block_on(async {
            let mut todo_list = TodoList::new();
            todo_list.add_item("Test task".to_string(), vec!["tag1".to_string()]);

            // Initialize SearchParams with appropriate values
            let search_params = SearchParams {
                words: vec![],
                tags: vec![Tag::new("tag1")],
            };

            let query = Query::Search(search_params);
            let result = run_query_async(query, &mut todo_list).await;
            assert!(result.is_ok());
            if let Ok(QueryResult::Found(results)) = result {
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].description, Description::new("Test task"));
            } else {
                panic!("Expected QueryResult::Found");
            }
        });
    }
}