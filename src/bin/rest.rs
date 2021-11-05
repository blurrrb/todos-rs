use todos::config::ApplicationConfig;
use todos::inmem_store::InMemStore;
use todos::postgres_store::Postgres;
use todos::todos::{Todo, TodoRepository};
use uuid::Uuid;

fn inject_db(database: &mut impl TodoRepository) -> Todo {
    let todo = Todo {
        id: Uuid::new_v4(),
        item: "Dependency injection testing".to_string(),
        completed: false,
    };
    database.create(todo)
}

fn run(config: &ApplicationConfig, todos_repository: &mut impl TodoRepository) {
    println!("{:?}", config);

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

    let todo3 = inject_db(todos_repository);

    println!("{:?}", todos_repository.get_all());

    todos_repository.delete(todo2.id);
    todos_repository.delete(todo.id);
    todos_repository.delete(todo3.id);

    println!("{:?}", config);
}

fn main() {
    let config = ApplicationConfig::new().unwrap();

    let mut todos_repository_pg = Postgres::new(&config.database_config);

    println!("postgres running");
    run(&config, &mut todos_repository_pg);

    let mut todos_repository_inmem = InMemStore::new();

    println!("inmem running");
    run(&config, &mut todos_repository_inmem);
}
