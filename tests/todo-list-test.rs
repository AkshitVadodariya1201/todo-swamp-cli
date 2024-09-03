#[cfg(test)]
mod tests {
    use todo_swamp::{SearchParams, SearchWord, Tag, TodoList};

    #[test]
    fn test_add_item() {
        let mut todo_list = TodoList::new();
        let description = "Test item".to_string();
        let tags = vec!["TestTag".to_string()];
        let index = todo_list.add_item(description.clone(), tags.clone());

        let item = todo_list.get_item(index).unwrap();
        assert_eq!(item.index.value(), index as u64);
        assert_eq!(item.description.value(), description);
        assert_eq!(item.tags[0].value(), tags[0]);
        assert!(!item.done);
    }

    #[test]
    fn test_get_item() {
        let mut todo_list = TodoList::new();
        let description = "Test item".to_string();
        let tags = vec!["TestTag".to_string()];
        let index = todo_list.add_item(description.clone(), tags.clone());

        let item = todo_list.get_item(index);
        assert!(item.is_some());
        let item = item.unwrap();
        assert_eq!(item.index.value(), index as u64);
        assert_eq!(item.description.value(), description);
        assert_eq!(item.tags[0].value(), tags[0]);
    }

    #[test]
    fn test_mark_done() {
        let mut todo_list = TodoList::new();
        let description = "Test item".to_string();
        let tags = vec!["TestTag".to_string()];
        let index = todo_list.add_item(description.clone(), tags.clone());

        todo_list.mark_done(index).unwrap();
        let item = todo_list.get_item(index).unwrap();
        assert!(item.done);
    }

    #[test]
    fn test_search() {
        let mut todo_list = TodoList::new();
        let description1 = "Test item 1".to_string();
        let tags1 = vec!["Tag1".to_string()];
        let description2 = "Test item 2".to_string();
        let tags2 = vec!["Tag2".to_string()];

        todo_list.add_item(description1.clone(), tags1.clone());
        let index2 = todo_list.add_item(description2.clone(), tags2.clone());

        // Mark the second item as done
        todo_list.mark_done(index2).unwrap();

        let params = SearchParams {
            words: vec![SearchWord::new("Test")],
            tags: vec![Tag::new("Tag1")],
        };

        let results = todo_list.search(params);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].description.value(), description1);
        assert_eq!(results[0].tags[0].value(), tags1[0]);
    }

    #[test]
    fn test_large_input() {
        use rayon::prelude::*;

        let mut todo_list = TodoList::new();
        let items: Vec<(String, Vec<String>)> = (0..5_000_000)
            .into_par_iter()
            .map(|i| {
                let description = format!("Test item {}", i);
                let tags = vec![format!("Tag{}", i)];
                (description, tags)
            })
            .collect();

        items.into_iter().for_each(|(description, tags)| {
            todo_list.add_item(description, tags);
        });

        assert_eq!(todo_list.items.len(), 5_000_000);
    }
}
