// use juniper::{EmptyMutation, RootNode};

// //the todo object
// pub struct AllTodo {
// title: String,
// description: String,
// }

// //making the work
// #[juniper::object(description = "A Todo App for Afrodew")]
// impl AllTodo {
// pub fn title(&self) -> String {
// self.title.to_string()
// }
// pub fn description(&self) -> String {
//     self.description.to_string()
//         } 
//     }

// //creating an array of a todo
// pub struct TodoApp;
// #[juniper::object]
// impl TodoApp {
// fn Todos() -> Vec<AllTodo> {
// vec![
//     AllTodo {
//         title: "Work".to_owned(),
//         description: "Go to work at 8:00 am".to_owned(),
//         }
//      ]
//   }  
//  }
        
// //exporting the schema
//  pub type Schema = RootNode<'static, TodoApp, EmptyMutation<()>>;  
//  pub fn create_schema() -> Schema {
//    Schema::new(TodoApp {}, EmptyMutation::new())
//  }




use juniper::FieldResult;
use juniper::{EmptySubscription, RootNode};

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Human {
    id: String,
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct NewHuman {
    name: String,
    appears_in: Vec<Episode>,
    home_planet: String,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn human(_id: String) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: "Luke".to_owned(),
            appears_in: vec![Episode::NewHope],
            home_planet: "Mars".to_owned(),
        })
    }
}

pub struct MutationRoot;

#[juniper::graphql_object]
impl MutationRoot {
    fn create_human(new_human: NewHuman) -> FieldResult<Human> {
        Ok(Human {
            id: "1234".to_owned(),
            name: new_human.name,
            appears_in: new_human.appears_in,
            home_planet: new_human.home_planet,
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}