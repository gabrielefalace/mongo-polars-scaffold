use crate::export::pipelines::{get_customer_ids_pipeline, other_pipeline};
use crate::utils::{dataframe_to_csv, pipeline_as_dataframe};
use mongodb::error::Result;
use polars::prelude::*;

pub async fn run() -> Result<()> {
    let mut first_frame =
        pipeline_as_dataframe("collection1", get_customer_ids_pipeline()).await?;

    // second pipeline not yet defined
    let second_frame = pipeline_as_dataframe("collection2", other_pipeline()).await?;


    let mut joined_frame = first_frame
        .lazy()
        .group_by([col("someAttribute")])
        .agg([col("someId").n_unique().alias("distinct_something_count")])
        .collect()
        .unwrap()
        .left_join(&second_frame, ["customerId"], ["_id"])
        .unwrap();
    dataframe_to_csv(&mut joined_frame, "result.csv")?;

    Ok(())
}
