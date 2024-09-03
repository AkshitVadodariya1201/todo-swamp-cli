use crate::*;
use async_std::task;

pub async fn run_line_async(line: &str, tl: &mut TodoList) {
    match parser::query(line) {
        Ok((_, q)) => match run_query_async(q, tl).await {
            Ok(r) => println!("{}", r),
            Err(e) => eprintln!("Error: {}", e),
        },
        Err(e) => eprintln!("Parsing error: {}", e),
    }
}

pub fn run_line(line: &str, tl: &mut TodoList) {
    task::block_on(run_line_async(line, tl));
}

pub async fn run_query_async(q: Query, tl: &mut TodoList) -> Result<QueryResult, QueryError> {
    match q {
        Query::Add(desc, tags) => {
            let tags: Vec<String> = tags.into_iter().map(|tag| tag.to_string()).collect();
            let item = tl.add_item(desc.to_string(), tags);
            tl.get_item(item)
                .map(|item| QueryResult::Added(item.clone()))
                .ok_or_else(|| QueryError("Item not found".to_owned()))
        }
        Query::Done(idx) => {
            let item = tl
                .get_item(idx.into())
                .ok_or_else(|| QueryError("Item not found".to_owned()))?;
            tl.mark_done(item.index.into())
                .ok_or_else(|| QueryError("Mark done error: Item not found".to_owned()))?;
            Ok(QueryResult::Done)
        }
        Query::Search(params) => {
            let results: Vec<todo_list::TodoItem> =
                tl.search(params).into_iter().cloned().collect();
            Ok(QueryResult::Found(results))
        }
    }
}
