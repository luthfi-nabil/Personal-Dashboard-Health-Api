use serde::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct InsulinUsage {
    #[serde(skip_deserializing)]
    pub insulin_usage_id: Uuid,
    pub insulin_assign_id: Uuid,
    pub units: f32,
    #[serde(skip_deserializing)]
    pub administered_at: NaiveDateTime,
    pub notes: Option<String>,
    #[serde(skip_deserializing)]
    pub is_active: i32,
    #[serde(skip_deserializing)]
    pub created_by: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsulinAssign {
    #[serde(skip_deserializing)]
    pub insulin_assign_id: Uuid,
    pub insulin_item_id: Uuid,
    pub batch_no: String,
    #[serde(skip_deserializing)]
    pub added_at: NaiveDateTime,
    pub notes: Option<String>,
    #[serde(skip_deserializing)]
    pub is_active: i32,
    #[serde(skip_deserializing)]
    pub created_by: String
}


#[derive(Debug, Serialize, Deserialize)]
pub struct InsulinAssignUsage {
    #[serde(skip_deserializing)]
    pub insulin_assign_id: Uuid,
    pub insulin_item_id: Uuid,
    pub batch_no: String,
    pub insulin_item_name: String,
    #[serde(skip_deserializing)]
    pub added_at: NaiveDateTime,
    pub notes: Option<String>,
    #[serde(skip_deserializing)]
    pub is_active: i32,
    pub total_units: f32,
    #[serde(skip_deserializing)]
    pub created_by: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsulinItem {
    #[serde(skip_deserializing)]
    pub insulin_item_id: Uuid,
    pub insulin_item_name: String,
    pub units: f32,
    pub uom: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
    pub notes: Option<String>,
    #[serde(skip_deserializing)]
    pub is_active: i32,
    #[serde(skip_deserializing)]
    pub created_by: String
}