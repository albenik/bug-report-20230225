use serde::{
    Deserialize,
    Serialize,
};
use surrealdb::engine::any::connect;

#[derive(Debug, Serialize, Deserialize)]
struct Entity {
    id: Option<String>,
    field: String,
}

#[tokio::main]
async fn main() {
    let db = connect("mem://").await.unwrap();

    let entity: Option<Entity> = db
        .update(("foo", "first"))
        .content(Entity {
            id: None,
            field: "value".to_string(),
        })
        .await
        .unwrap();

    println!("{entity:?}");
}
