
use juniper::FieldResult;
use juniper::{EmptySubscription, RootNode};
use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};


// Query Types
#[derive(GraphQLObject)]
#[graphql(description = "A todo")]
struct Todo {
    id: String,
    content: String,
    completed_at: Option<String>,
    created_at: String
}



// Input Types
#[derive(GraphQLInputObject)]
#[graphql(description = "This is the input when creating a new todo")]
struct CreateTodoInput {
    content: String,
    created_at: String
}



// Queries
pub struct Query;

#[juniper::graphql_object]
impl Query {
    fn todos() -> FieldResult<Vec<Todo>> {
        Ok(vec![
            Todo{id: "t1".to_string(), content: "Build house".to_string(), completed_at: None, created_at: "7/6/2021".to_string()},
            Todo{id: "t2".to_string(), content: "Start company".to_string(), completed_at: Some("7/6/2021".to_string()), created_at: "7/6/2021".to_string()},
            Todo{id: "t3".to_string(), content: "Improve Africa".to_string(), completed_at: None, created_at: "7/6/2021".to_string()},
            Todo{id: "t4".to_string(), content: "Get a baby".to_string(), completed_at: None, created_at: "7/6/2021".to_string()},
        ])
    }
}



// Mutations
pub struct Mutation;

#[juniper::graphql_object]
impl Mutation {
    fn create_todo(todo: CreateTodoInput) -> FieldResult<Todo> {
        Ok(Todo{id: "t5".to_string(), content: todo.content, completed_at: None, created_at: todo.created_at})
    }
}



// Schema
pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, EmptySubscription::new())
}