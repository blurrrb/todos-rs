use super::Postgres;
use crate::todos::{Todo, TodoRepository};
use diesel::{QueryDsl, RunQueryDsl};
use uuid::Uuid;

use crate::schema::todos::dsl::todos;

impl TodoRepository for Postgres {
    fn get_all(&self) -> Vec<Todo> {
        match todos.load(&self.get_connection()) {
            Ok(todos_list) => todos_list,
            Err(_) => vec![],
        }
    }

    fn get_by_id(&self, id: Uuid) -> Option<Todo> {
        match todos.find(id).first(&self.get_connection()) {
            Ok(todo) => Some(todo),
            Err(_) => None,
        }
    }

    fn create(&self, todo: Todo) -> Todo {
        diesel::insert_into(todos)
            .values(&todo)
            .get_result(&self.get_connection())
            .expect("Error saving new todo")
    }

    fn update(&self, todo: Todo) -> Todo {
        diesel::update(todos.find(todo.id))
            .set(&todo)
            .get_result(&self.get_connection())
            .expect("Error updating todo")
    }

    fn delete(&self, id: Uuid) -> bool {
        diesel::delete(todos.find(id))
            .execute(&self.get_connection())
            .is_ok()
    }
}
