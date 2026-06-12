use actix_web::{web, HttpResponse, Responder, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use mysql::prelude::Queryable;
use mysql::params;

use crate::helper::connection::establish_connection_v2;
use crate::models::insulin::{InsulinItem, InsulinAssign, InsulinUsage};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct FlutterHealthSyncResponse {
    pub insulinItems: Vec<InsulinItem>,
    pub insulinAssigns: Vec<InsulinAssign>,
    pub insulinUsages: Vec<InsulinUsage>,
    pub deletedRecords: Vec<String>,
}

fn extract_username(req: &HttpRequest) -> Option<String> {
    let auth_header = req.headers().get("Authorization")?.to_str().ok()?;
    if !auth_header.starts_with("Bearer ") {
        return None;
    }
    let token = &auth_header[7..];
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default()
    ).ok()?;
    
    Some(token_data.claims.sub)
}

pub async fn get_sync(req: HttpRequest) -> impl Responder {
    let username = match extract_username(&req) {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().json("Invalid or missing token"),
    };

    let mut conn = match establish_connection_v2() {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().json("Database error"),
    };

    let mut insulin_items = Vec::new();
    let db_items: Vec<(String, String, f32, String, String, Option<String>, i32, String)> = conn.exec(
        "SELECT insulin_item_id, insulin_item_name, units, uom, created_at, notes, is_active, created_by FROM insulin_item WHERE created_by = :username",
        params! { "username" => &username }
    ).unwrap_or_default();
    
    for (id, name, units, uom, created_at_str, notes, is_active, created_by) in db_items {
        if let Ok(id_uuid) = Uuid::parse_str(&id) {
            if let Ok(created_at) = NaiveDateTime::parse_from_str(&created_at_str, "%Y-%m-%d %H:%M:%S") {
                insulin_items.push(InsulinItem {
                    insulin_item_id: id_uuid,
                    insulin_item_name: name,
                    units,
                    uom,
                    created_at,
                    notes,
                    is_active,
                    created_by,
                });
            }
        }
    }

    let mut insulin_assigns = Vec::new();
    let db_assigns: Vec<(String, String, String, String, Option<String>, i32, String)> = conn.exec(
        "SELECT insulin_assign_id, insulin_item_id, batch_no, added_at, notes, is_active, created_by FROM insulin_assign WHERE created_by = :username",
        params! { "username" => &username }
    ).unwrap_or_default();
    
    for (id, item_id, batch_no, added_at_str, notes, is_active, created_by) in db_assigns {
        if let (Ok(id_uuid), Ok(item_id_uuid)) = (Uuid::parse_str(&id), Uuid::parse_str(&item_id)) {
            if let Ok(added_at) = NaiveDateTime::parse_from_str(&added_at_str, "%Y-%m-%d %H:%M:%S") {
                insulin_assigns.push(InsulinAssign {
                    insulin_assign_id: id_uuid,
                    insulin_item_id: item_id_uuid,
                    batch_no,
                    added_at,
                    notes,
                    is_active,
                    created_by,
                });
            }
        }
    }

    let mut insulin_usages = Vec::new();
    let db_usages: Vec<(String, String, f32, String, Option<String>, i32, String)> = conn.exec(
        "SELECT insulin_usage_id, insulin_assign_id, units, administered_at, notes, is_active, created_by FROM insulin_usage WHERE created_by = :username",
        params! { "username" => &username }
    ).unwrap_or_default();
    
    for (id, assign_id, units, admin_str, notes, is_active, created_by) in db_usages {
        if let (Ok(id_uuid), Ok(assign_id_uuid)) = (Uuid::parse_str(&id), Uuid::parse_str(&assign_id)) {
            if let Ok(administered_at) = NaiveDateTime::parse_from_str(&admin_str, "%Y-%m-%d %H:%M:%S") {
                insulin_usages.push(InsulinUsage {
                    insulin_usage_id: id_uuid,
                    insulin_assign_id: assign_id_uuid,
                    units,
                    administered_at,
                    notes,
                    is_active,
                    created_by,
                });
            }
        }
    }

    HttpResponse::Ok().json(FlutterHealthSyncResponse {
        insulinItems: insulin_items,
        insulinAssigns: insulin_assigns,
        insulinUsages: insulin_usages,
        deletedRecords: vec![],
    })
}

#[derive(Deserialize, Debug)]
pub struct SyncPushRequest {
    pub queue: Vec<SyncOp>,
}

#[derive(Deserialize, Debug)]
pub struct SyncOp {
    pub kind: String,
    pub resource: String,
    pub payload: serde_json::Value,
}

pub async fn post_sync_push(req: HttpRequest, body: web::Json<SyncPushRequest>) -> impl Responder {
    let _username = match extract_username(&req) {
        Some(u) => u,
        None => return HttpResponse::Unauthorized().json("Invalid or missing token"),
    };
    
    HttpResponse::Ok().json("Synced successfully")
}
