use crate::db;
use futures::stream::TryStreamExt;
use mongodb::bson::Document;
use mongodb::Collection;
use polars::io::csv::write::CsvWriter;
use polars::io::json::JsonReader;
use polars::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;
use std::fs::File;
use std::io::Cursor;

pub const TEST_IDS_TO_SKIP: &[&str] = &[
    "id1",
    "id2",
    "id3"
];

pub async fn find_as_dataframe<T>(
    collection_name: &str,
    query: Document,
) -> mongodb::error::Result<DataFrame>
where
    T: DeserializeOwned + Unpin + Send + Sync + Serialize,
{
    let db = db::get_db().await?;

    let collection: Collection<T> = db.collection::<T>(collection_name);
    let mut mongo_cursor = collection.find(query).await?;
    let documents: Vec<T> = mongo_cursor.try_collect().await?;
    let json = serde_json::to_string(&documents).unwrap();
    let json_cursor = Cursor::new(json);
    let df = JsonReader::new(json_cursor).finish().unwrap();
    Ok(df)
}

pub async fn pipeline_as_dataframe<T>(
    collection_name: &str,
    pipeline: Vec<Document>,
) -> mongodb::error::Result<DataFrame>
where
    T: DeserializeOwned + Serialize + Unpin + Send + Sync,
    Vec<T>: Extend<Document>,
{
    // let db = db::get_db().await?;
    let db = db::get_db_once().await.clone();

    let collection: Collection<T> = db.collection::<T>(collection_name);

    // Execute aggregation pipeline
    let mut mongo_cursor = collection.aggregate(pipeline).await?;

    // Collect results into a Vec<T>
    let documents: Vec<T> = mongo_cursor.try_collect().await?;

    // Serialize to JSON string
    let json = serde_json::to_string(&documents).unwrap();

    // Convert JSON to a Polars DataFrame
    let json_cursor = Cursor::new(json);
    let df = JsonReader::new(json_cursor).finish().unwrap();

    Ok(df)
}

pub fn dataframe_to_csv(mut frame: &mut DataFrame, file_name: &str) -> mongodb::error::Result<()> {
    let file = File::create(file_name)?;
    CsvWriter::new(file)
        .include_header(true)
        .with_separator(b',')
        .finish(&mut frame)
        .expect("Something unexpected happened writing the DataFrame to CSV");

    Ok(())
}

