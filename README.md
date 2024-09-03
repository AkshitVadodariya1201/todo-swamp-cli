# Todo Swamp

Todo Swamp is a Rust-based command-line application for managing a todo list. It supports adding, searching, and marking todo items as done. The application is designed to handle large inputs efficiently.

## Features

- **Add Todo Items**: Add new todo items with descriptions and tags.
- **Search Todo Items**: Search for todo items based on words and tags.
- **Mark Items as Done**: Mark todo items as completed.
- **Display Todo Items**: Display todo items with their details.

## Project Structure

The project is structured as follows:
```bash

 ├── .gitignore 
 ├── Cargo.lock 
 ├── Cargo.toml 
 ├── README.md 
 ├── rust-toolchain 
 ├── src/ 
 │ ├── bin/ 
 │ │ └── application.rs 
 │ ├── lib.rs 
 │ ├── parser.rs 
 │ ├── query.rs 
 │ ├── runner.rs 
 │ └── todo_list.rs 
 ├── tests/ 
 │ ├── fixtures/ 
 │ │ ├── Bishibosh (1).out 
 │ │ ├── Bishibosh.in
 │ │ ├── FlamespFlamespike-The-CrawlerFlamespike-The-Crawlerike-The-Crawler.in
 │ │ ├── FlamespFlamespike-The-CrawlerFlamespike-The-Crawlerike-The-Crawler.out
 │ │ ├── sample.in
 │ │ └── sample.out
 │ ├── parser-test.rs 
 │ ├── query-test.rs 
 │ ├── runner-test.rs 
 │ ├── standard-test.rs 
 │ ├── test-cases.rs 
 │ └── todo-list-test.rs
```
## Code Overview

The code is divided into the following modules:

### `src/todo_list.rs`

This file contains the core data structures and implementations for the todo list, including:

- **Index**: Represents the index of a todo item.
- **Description**: Represents the description of a todo item.
- **Tag**: Represents a tag associated with a todo item.
- **TodoItem**: Represents a todo item.
- **TodoList**: Manages a list of todo items.

### `src/parser.rs`

This file contains functions for parsing commands and inputs.

### `src/query.rs`

This file contains the logic for querying and searching todo items.

### `src/runner.rs`

This file contains the main logic for running the application.

### `tests/`

This directory contains test files for various modules of the application.
 
## Getting Started

### Prerequisites

- Rust (latest stable version)

### Building the Project

To build the project, run:

```sh
cargo build
```

### Running the Project

To run the project, use the following command:

```sh
cargo run
```

### Running Tests

To run the tests, use the following command:

```sh
cargo test
```

## Usage

The application supports the following commands:

- `add`: Add a new todo item.
- `search`: Search for todo items based on words and tags.
- `done`: Mark a todo item as completed.
  
### Adding a Todo Item

To add a new todo item, use the following command:

```sh
add "Buy groceries"  #shopping, #groceries
```

### Searching for Todo Items

To search for todo items, use the following command:

```sh
search groceries
```

### Marking a Todo Item as Done

To mark a todo item as completed, use the following command:

```sh
done 0
```

> [!NOTE]  
> After running cargo run, always first enter the number that represents how many commands you want to run. 
> ```sh
> 3
> add "Buy groceries"  #shopping, #groceries
> search groceries
> done 0
> ```
> This will add a todo item, search for todo items with the tag "groceries", and mark the first item as done.


## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

Happy coding! 🚀