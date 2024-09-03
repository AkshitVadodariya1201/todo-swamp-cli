use std::fmt;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Query {
    Add(Description, Vec<Tag>),
    Done(Index),
    Search(SearchParams),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchParams {
    pub words: Vec<SearchWord>,
    pub tags: Vec<Tag>,
}
impl SearchParams {
    pub fn matches(&self, ti: &todo_list::TodoItem) -> bool {
        // Check if all search words are subsequences in the description within individual words
        let description_matches = self
            .words
            .iter()
            .all(|w| is_subsequence_in_any_word(&w.0, ti.description.value()));

        // Check if all tags in search params are subsequences in any of the tags of the item
        let tags_match = self.tags.iter().all(|search_tag| {
            ti.tags
                .iter()
                .any(|item_tag| is_subsequence(&search_tag.value(), &item_tag.value()))
        });

        description_matches && tags_match
    }
}

pub fn is_subsequence_in_any_word(search_word: &str, target: &str) -> bool {
    let search_chars: Vec<char> = search_word.chars().collect();

    target.split_whitespace().any(|word| {
        let mut search_index = 0;

        for ch in word.chars() {
            if search_index < search_chars.len() && ch == search_chars[search_index] {
                search_index += 1;
            }
        }

        // If the entire search word is found within this word, return true
        search_index == search_chars.len()
    })
}

pub fn is_subsequence(search_word: &str, target: &str) -> bool {
    let search_chars: Vec<char> = search_word.chars().collect();
    let mut search_index = 0;

    for ch in target.chars() {
        if search_index < search_chars.len() && ch == search_chars[search_index] {
            search_index += 1;
        }
    }

    // Return true if we successfully matched all characters in the search_word
    search_index == search_chars.len()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchWord(pub String);
impl SearchWord {
    pub fn new(s: &str) -> SearchWord {
        SearchWord(s.to_owned())
    }
}

impl fmt::Display for SearchWord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryResult {
    Added(TodoItem),
    Done,
    Found(Vec<todo_list::TodoItem>),
}

impl fmt::Display for QueryResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            QueryResult::Added(ti) => write!(f, "{}", ti.index),
            QueryResult::Done => write!(f, "done"),
            QueryResult::Found(rs) => {
                let mut buff: Vec<String> = vec![];
                buff.push(format!("{} item(s) found", rs.len()));
                for i in rs {
                    let tags = i
                        .tags
                        .iter()
                        .map(|tag| format!("#{}", tag.value()))
                        .collect::<Vec<_>>()
                        .join(" ");
                    buff.push(format!("{} \"{}\" {}", i.index, i.description, tags));
                }
                write!(f, "{}", buff.join("\n"))
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryError(pub String);

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "An error occurred while processing the query: {}.",
            self.0
        )
    }
}
