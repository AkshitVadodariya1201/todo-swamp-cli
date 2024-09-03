use todo_swamp::TodoList;
use std::io::{self, BufRead};
use todo_swamp::runner::run_line;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the number of queries
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Initialize the TodoList
    let mut todo_list = TodoList::new();

    // Process each query
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            run_line(&line, &mut todo_list);
        }
    }
}