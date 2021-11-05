use uuid::Uuid;

use super::InMemStore;
use crate::todos::{Todo, TodoRepository};

impl TodoRepository for InMemStore {
    fn get_all(&self) -> Vec<Todo> {
        self.store.values().cloned().collect()
    }

    fn get_by_id(&self, id: Uuid) -> Option<Todo> {
        self.store.get(&id).cloned()
    }

    fn create(&mut self, todo: Todo) -> Todo {
        let id = todo.id.clone();
        self.store.insert(todo.id, todo);
        self.store.get(&id).cloned().unwrap()
    }

    fn update(&mut self, todo: Todo) -> Todo {
        let id = todo.id.clone();
        self.store.insert(todo.id, todo);
        self.store.get(&id).cloned().unwrap()
    }

    fn delete(&mut self, id: Uuid) -> bool {
        self.store.remove(&id).is_some()
    }
}
