use anyhow::{Ok, Result};
use dotenv::dotenv;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::{
    CollectionOperationResponse, Condition, CreateCollection, Filter, SearchPoints, VectorParams,
    VectorsConfig,
};
use serde_json::json;
use serde_json::Value;
use std::env;

async fn make_client() -> Result<QdrantClient> {
    dotenv().ok();

    let mut config = QdrantClientConfig::from_url(env::var("SERVER_URL").unwrap().as_str());

    let api_key = env::var("API_KEY").unwrap();

    config.set_api_key(&api_key);
    config.keep_alive_while_idle = true;

    dbg!(api_key.clone());

    println!("config.uri {}", config.uri);

    let client = QdrantClient::new(Some(config))?;

    Ok(client)
}

async fn create_collection(client: &QdrantClient, collection_name: &str) -> Result<()> {
    client
        .create_collection(&CreateCollection {
            collection_name: collection_name.into(),
            vectors_config: Some(VectorsConfig {
                config: Some(Config::Params(VectorParams {
                    size: 10,
                    distance: Distance::Cosine.into(),
                    ..Default::default()
                })),
            }),
            ..Default::default()
        })
        .await?;

    // let collection_info = client.collection_info(collection_name).await?;
    // dbg!(collection_info);

    let payload: Payload = json!(
        {
            "foo": "Bar",
            "bar": 12,
            "baz": {
                "qux": "quux"
            }
        }
    )
    .try_into()
    .unwrap();

    let points = vec![PointStruct::new(0, vec![12.; 10], payload)];
    client
        .upsert_points_blocking(collection_name, points, None)
        .await?;

    Ok(())
}

async fn get_collections_list(client: &QdrantClient) -> Result<()> {
    let collections_list = client.list_collections().await?;
    dbg!(collections_list);

    Ok(())
}

async fn delete_collection(
    client: &QdrantClient,
    collection_name: &str,
) -> Result<CollectionOperationResponse> {
    let result: CollectionOperationResponse = client.delete_collection(collection_name).await?;

    Ok(result)
}

async fn search_point(
    client: &QdrantClient,
    collection_name: &str,
    search_field: &str,
) -> Result<Value> {
    let search_result = client
        .search_points(&SearchPoints {
            collection_name: collection_name.into(),
            vector: vec![11.; 10],
            filter: Some(Filter::all([Condition::matches(search_field, 12)])),
            limit: 10,
            with_payload: Some(true.into()),
            ..Default::default()
        })
        .await?;
    // dbg!(&search_result);

    let found_point = search_result.result.into_iter().next().unwrap();
    let mut payload = found_point.payload;
    let baz_payload = payload.remove("baz").unwrap().into_json();
    println!("baz: {}", baz_payload); // baz: {"qux":"quux"}

    Ok(baz_payload)
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = make_client().await?;

    let collection_name = "test";

    create_collection(&client, collection_name).await?;

    get_collections_list(&client).await?;

    search_point(&client, collection_name, "bar").await?;

    // delete_collection(&client, collection_name).await?;

    Ok(())
}
