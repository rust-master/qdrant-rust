use std::env;

use anyhow::{Ok, Result};
use dotenv::dotenv;
use serde_json::{json, Value};

use qdrant_client::{
    Payload,
    Qdrant,
};

use qdrant_client::qdrant::{
    CollectionOperationResponse,
    Condition,
    CreateCollectionBuilder,
    Distance,
    Filter,
    PointStruct,
    SearchPointsBuilder,
    VectorParamsBuilder,
    UpsertPointsBuilder,
};

async fn make_client() -> Result<Qdrant> {
    dotenv().ok();

    let server_url = env::var("SERVER_URL").expect("SERVER_URL must be set");
    let api_key = env::var("API_KEY").expect("API_KEY must be set");

    let client = Qdrant::from_url(&server_url)
        .api_key(api_key)
        .build()?;

    Ok(client)
}

async fn create_collection(client: &Qdrant, collection_name: &str) -> Result<()> {
    client.create_collection(
        CreateCollectionBuilder::new(collection_name)
            .vectors_config(VectorParamsBuilder::new(10, Distance::Cosine)),
    )
    .await?;

    // let collection_info = client.collection_info(collection_name).await?;
    // dbg!(collection_info);

    let payload: Payload = json!({
        "foo": "Bar",
        "bar": 12,
        "baz": {
            "qux": "quux"
        }
    })
    .try_into()
    .unwrap();

    let points = vec![PointStruct::new(0, vec![12.; 10], payload)];

    client
        .upsert_points(UpsertPointsBuilder::new(collection_name, points).wait(true))
        .await?;

    Ok(())
}

async fn get_collections_list(client: &Qdrant) -> Result<()> {
    let collections_list = client.list_collections().await?;
    dbg!(collections_list);
    Ok(())
}

async fn search_point(
    client: &Qdrant,
    collection_name: &str,
    search_field: &str,
) -> Result<Value> {
    let search_request = SearchPointsBuilder::new(
        collection_name,
        vec![11.; 10],
        10,
    )
    .filter(Filter::all([Condition::matches(search_field, 12)]))
    .with_payload(true);

    let search_result = client.search_points(search_request).await?;

    // dbg!(&search_result);

    let found_point = search_result.result.into_iter().next().unwrap();
    let mut payload = found_point.payload;
    let baz_payload = payload.remove("baz").unwrap().into_json();

    println!("baz: {}", baz_payload); // baz: {"qux":"quux"}

    Ok(baz_payload)
}

async fn delete_collection(
    client: &Qdrant,
    collection_name: &str,
) -> Result<CollectionOperationResponse> {
    let result: CollectionOperationResponse = client.delete_collection(collection_name).await?;
    Ok(result)
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = make_client().await?;

    let collection_name = "test";
    create_collection(&client, collection_name).await?;
    get_collections_list(&client).await?;
    search_point(&client, collection_name, "bar").await?;
    delete_collection(&client, collection_name).await?;

    Ok(())
}
