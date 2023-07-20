/*
 * Revolt API
 *
 * Open source user-first chat platform.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: contact@revolt.chat
 * Generated by: https://openapi-generator.tech
 */

/// CollectionScans : Query collection scan stats



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CollectionScans {
    /// Number of total collection scans
    #[serde(rename = "total")]
    pub total: i64,
    /// Number of total collection scans not using a tailable cursor
    #[serde(rename = "nonTailable")]
    pub non_tailable: i64,
}

impl CollectionScans {
    /// Query collection scan stats
    pub fn new(total: i64, non_tailable: i64) -> CollectionScans {
        CollectionScans {
            total,
            non_tailable,
        }
    }
}


