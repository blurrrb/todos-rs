use todos::config::ApplicationConfig;
use todos::postgres::Postgres;
use todos::todos::{Todo, TodoRepository};
use uuid::Uuid;

fn main() {
    let config = ApplicationConfig::new().unwrap();
    println!("{:?}", config);

    let todos_repository: Postgres = Postgres::new(&config.database);

    let todo = Todo {
        id: Uuid::new_v4(),
        item: String::from("Learn Rust"),
        completed: false,
    };

    let mut todo = todos_repository.create(todo);

    println!("{:?}", todo);

    todo.completed = true;

    let todo = todos_repository.update(todo);

    println!("{:?}", todo);

    let todo2 = todos_repository.create(Todo {
        id: Uuid::new_v4(),
        item: String::from("Create Todo rest api"),
        completed: false,
    });

    println!("{:?}", todos_repository.get_all());

    todos_repository.delete(todo2.id);
    todos_repository.delete(todo.id);

    println!("{:?}", config);
}
