use mysql::{Result, PooledConn, params, Error as MysqlError};
use mysql::prelude::*;
use std::collections::HashMap;
use chrono::{Utc, DateTime, NaiveDateTime};
use crate::models::responses::{DatabaseResult};
use crate::models::insulin::{InsulinAssign, InsulinItem, InsulinUsage};
use uuid::Uuid;
use std::error::Error;

pub fn create_insulin_usage_table(conn: &mut PooledConn) -> Result<()> {
    conn.query_drop(
        "CREATE TABLE IF NOT EXISTS insulin_usage (
            insulin_usage_id CHAR(36) PRIMARY KEY,
            insulin_item_id VARCHAR(255) NOT NULL,
            units FLOAT NOT NULL,
            administered_at DATETIME NOT NULL,
            notes TEXT,
            is_active INT NOT NULL
        )"
    )?;
    Ok(())
}

pub fn create_insulin_assign_table(conn: &mut PooledConn) -> Result<()> {
    conn.query_drop(
        "CREATE TABLE IF NOT EXISTS insulin_assign (
            insulin_assign_id CHAR(36) PRIMARY KEY,
            insulin_item_id CHAR(36) NOT NULL,
            added_at DATETIME NOT NULL,
            notes TEXT,
            is_active INT NOT NULL
        )"
    )?;
    Ok(())
}

pub fn create_insulin_item_table(conn: &mut PooledConn) -> Result<()> {
    conn.query_drop(
        "CREATE TABLE IF NOT EXISTS insulin_item (
            insulin_item_id CHAR(36) PRIMARY KEY,
            insulin_item VARCHAR(255) NOT NULL,
            units FLOAT NOT NULL,
            uom VARCHAR(255) NOT NULL,
            created_at DATETIME NOT NULL,
            notes TEXT,
            is_active INT NOT NULL
        )"
    )?;
    Ok(())
}

pub fn select_insulin_item(conn: &mut PooledConn, insulin_item_id: &str) -> Result<Vec<InsulinItem>, Box<dyn Error>> {
    let query = r#"
        SELECT insulin_item_id, insulin_item, units, uom, created_at, notes, is_active
        FROM insulin_item
        WHERE insulin_item_id = :id AND is_active = 1
    "#;

    let result: Vec<InsulinItem> = conn.exec_map(
        query,
        params! { "id" => insulin_item_id },
        |(insulin_item_id, insulin_item, units, created_at, notes, is_active): (String, String, f32, String, Option<String>, i32)| {
            InsulinItem {
                insulin_item_id: Uuid::parse_str(&insulin_item_id)
                .unwrap_or_else(|_| Uuid::nil()),
                insulin_item: insulin_item,
                units: units,
                created_at: NaiveDateTime::parse_from_str(&created_at, "%Y-%m-%d %H:%M:%S")
                .unwrap_or_else(|_| NaiveDateTime::from_timestamp_opt(0, 0).unwrap()),
                is_active: is_active,
                notes: notes,
            }
        },
    )?;

    Ok(result)
}

pub fn select_all_insulin_item(conn: &mut PooledConn) -> Result<Vec<InsulinItem>, Box<dyn Error>> {
    let query = r#"
        SELECT insulin_item_id, insulin_item, units, uom, created_at, notes, is_active
        FROM insulin_item
    "#;

    let result: Vec<InsulinItem> = conn.query_map(
        query,
        |(insulin_item_id, insulin_item, units,uom, created_at, notes, is_active): (String, String, f32, String, NaiveDateTime, Option<String>, i32)| {
            InsulinItem {
                insulin_item_id: Uuid::parse_str(&insulin_item_id)
                .unwrap_or_else(|_| Uuid::nil()),
                insulin_item: insulin_item,
                units: units,
                uom: uom,
                notes: notes,
                created_at: created_at,
                is_active: is_active
            }
        },
    )?;

    Ok(result)
}

pub fn insert_insulin_item(conn: &mut PooledConn, insulin_item: &InsulinItem) -> Result<DatabaseResult, Box<dyn Error>> {
    let query = r#"
        INSERT INTO insulin_item 
        (insulin_item_id, insulin_item, units, uom, created_at, notes, is_active)
        VALUES 
        (:id, :cat, :units, :uom, :created_at, :notes, :is_active)
    "#;

    let result = conn.exec_drop(
        query,
        params! {
            "id" => insulin_item.insulin_item_id.to_string(),
            "cat" => &insulin_item.insulin_item,
            "units" => insulin_item.units,
            "uom" => &insulin_item.uom,
            "created_at" => insulin_item.created_at,
            "notes" => &insulin_item.notes,
            "is_active" => insulin_item.is_active,
        },
    );
    match result {
        Ok(_) => Ok(DatabaseResult::Inserted),

        Err(MysqlError::MySqlError(ref e)) => match e.code {
            1062u16 => {
                Ok(DatabaseResult::Duplicate)
            }
            _ => Err(Box::new(MysqlError::MySqlError(e.clone()))),
        },

        Err(e) => Err(Box::new(e)),
    }
}

pub fn delete_insulin_item(conn: &mut PooledConn, insulin_item_id: &str) -> Result<(), Box<dyn Error>> {
    conn.exec_drop(
        "UPDATE insulin_item SET is_active = 0 WHERE insulin_item_id = :id",
        params! { "id" => insulin_item_id },
    )?;
    Ok(())
}