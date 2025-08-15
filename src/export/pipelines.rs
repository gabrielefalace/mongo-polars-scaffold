use crate::utils::TEST_IDS_TO_SKIP;
use mongodb::bson;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use std::collections::HashSet;

pub fn get_customer_ids_pipeline(company_ids:Vec<String> ) -> Vec<bson::Document> {
    vec![
        doc! { "$match": {
                    "companyIds": {"$in": company_ids},
                    "someOtherId": { "$nin": TEST_IDS_TO_SKIP }
                }
        },
        doc! {
            "$project": {
                    "customerId": 1,
                    "_id": 0
                }
        },
    ]
}
