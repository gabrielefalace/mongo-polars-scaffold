mod db;
mod export;
mod utils;

use crate::export::export_contacts;
use crate::export::export_funds;
use crate::export::export_manco_users;
use crate::export::export_subscriptions;
use crate::export::export_investors;
use crate::export::export_distinct_investors_per_fund;
use crate::export::export_members_per_fund;


use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use polars::prelude::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run <export_type>; example: cargo run export1");
        return;
    }

    let export_type = args[1].as_str();

    match export_type {
        "export1" => export::run().await.unwrap(),
        // extend here
        _ => eprintln!("Unknown export type! Use names matching the export_type values here, such as 'export1' ... "),
    }
}



