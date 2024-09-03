use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::{convert::TryInto, fmt};

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Index(u64);

impl Index {
    pub fn new(i: u64) -> Index {
        Index(i)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

impl Into<usize> for Index {
    fn into(self) -> usize {
        self.0.try_into().unwrap()
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Description(String);

impl Description {
    pub fn new(s: &str) -> Description {
        Description(s.to_owned())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Description {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag(String);

impl Hash for Tag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Tag {
    pub fn new(s: &str) -> Tag {
        Tag(s.to_owned())
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn from_strings(ss: Vec<&str>) -> Vec<Tag> {
        ss.clone().into_iter().map(|s| Tag::new(s)).collect()
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoItem {
    pub index: Index,
    pub description: Description,
    pub tags: Vec<Tag>,
    pub done: bool,
}

impl TodoItem {
    pub fn new(index: usize, description: Description, tags: Vec<Tag>, done: bool) -> TodoItem {
        TodoItem {
            index: Index::new(index as u64),
            description,
            tags,
            done,
        }
    }
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}, {:?}", self.index, self.description, self.tags)
    }
}

#[derive(Debug)]
pub struct TodoList {
    pub items: Vec<TodoItem>,
    index_map: HashMap<usize, usize>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            items: Vec::new(),
            index_map: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, desc: String, tags: Vec<String>) -> usize {
        let index = self.items.len();
        let description = Description::new(&desc);
        let tag_objects: Vec<Tag> = tags.into_iter().map(|arg0| Tag::new(&arg0)).collect();
        self.items
            .push(TodoItem::new(index, description, tag_objects, false));
        self.index_map.insert(index, self.items.len() - 1);
        index
    }

    pub fn get_item(&self, index: usize) -> Option<&TodoItem> {
        self.index_map
            .get(&index)
            .and_then(|&pos| self.items.get(pos))
    }

    pub fn mark_done(&mut self, index: usize) -> Option<()> {
        if let Some(&pos) = self.index_map.get(&index) {
            self.items[pos].done = true;
            Some(())
        } else {
            None
        }
    }

    pub fn search(&self, params: SearchParams) -> Vec<&TodoItem> {
        self.items
            .iter()
            .filter(|item| !item.done && params.matches(item))
            .collect()
    }
}
