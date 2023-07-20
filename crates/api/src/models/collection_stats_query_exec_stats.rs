/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// CollectionStatsQueryExecStats : Query exec stats



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CollectionStatsQueryExecStats {
    #[serde(rename = "collectionScans")]
    pub collection_scans: Box<crate::models::QueryExecStatsCollectionScans>,
}

impl CollectionStatsQueryExecStats {
    /// Query exec stats
    pub fn new(collection_scans: crate::models::QueryExecStatsCollectionScans) -> CollectionStatsQueryExecStats {
        CollectionStatsQueryExecStats {
            collection_scans: Box::new(collection_scans),
        }
    }
}


