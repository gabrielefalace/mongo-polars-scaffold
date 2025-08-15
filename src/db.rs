use mongodb::options::ClientOptions;
use mongodb::{Client, Database};
use tokio::sync::OnceCell;

static DB: OnceCell<Database> = OnceCell::const_new();


pub async fn get_db() -> mongodb::error::Result<Database> {
    let database_url = "mongodb://localhost:27017/?directConnection=true";
    println!(">>> Connecting to DB: {:?}", database_url);
    let client_options = ClientOptions::parse(database_url).await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("my-db-name");
    Ok(db)
}


pub async fn get_db_once() -> &'static Database {
    DB.get_or_init(|| async {
        let url = "mongodb://localhost:27017/?directConnection=true";
        println!(">>> Connecting to DB: {url}");
        let client = Client::with_uri_str(url).await.expect("Mongo connection failed");
        client.database("my-db-name")
    }).await
}
