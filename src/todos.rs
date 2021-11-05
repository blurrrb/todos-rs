use crate::schema::todos;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "todos"]
pub struct Todo {
    pub id: Uuid,
    pub item: String,
    pub completed: bool,
}

pub trait TodoRepository {
    fn get_all(&self) -> Vec<Todo>;
    fn get_by_id(&self, id: Uuid) -> Option<Todo>;
    fn create(&self, todo: Todo) -> Todo;
    fn update(&self, todo: Todo) -> Todo;
    fn delete(&self, id: Uuid) -> bool;
}
