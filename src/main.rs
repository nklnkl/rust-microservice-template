#[macro_use] extern crate rocket;
#[macro_use] extern crate juniper;

mod models;

use juniper::{EmptyMutation, RootNode, EmptySubscription};
use rocket::State;
use juniper_rocket::{GraphQLRequest, GraphQLResponse};
use rocket::response::content::RawHtml;
use models::test_object::TestObject;

struct Context;
impl juniper::Context for Context {}

struct Query;

#[graphql_object(Context = Context)]
impl Query {
    fn hello() -> &'static str {
        "Hello, GraphQL!"
    }

    fn test_object(id: String) -> Option<TestObject> {
        // This is a mock implementation. In a real application, you would fetch the object from a database.
        Some(TestObject::new("Test Object".to_string()))
    }
}

type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rust Rocket Microservice Template!!"
}

#[get("/graphql")]
fn graphiql() -> RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[post("/graphql", data = "<request>")]
fn graphql_handler(
    context: &State<Context>,
    schema: &State<Schema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    request.execute_sync(schema, context)
}

#[launch]
fn rocket() -> _ {
    let schema = Schema::new(Query, EmptyMutation::new(), EmptySubscription::new());
    rocket::build()
        .manage(Context)
        .manage(schema)
        .mount("/", routes![index, graphiql, graphql_handler])
        .configure(rocket::Config::figment().merge(("address", "0.0.0.0")))
}
