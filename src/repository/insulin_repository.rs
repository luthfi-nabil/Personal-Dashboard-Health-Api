use mysql::{Result, PooledConn, params, Error as MysqlError};
use mysql::prelude::*;
use std::collections::HashMap;
use chrono::{Utc, DateTime, NaiveDateTime};
use crate::models::responses::{DatabaseResult};
use crate::models::insulin::{InsulinAssign, InsulinItem, InsulinUsage, InsulinAssignUsage};
use uuid::Uuid;
use std::error::Error;

pub fn create_insulin_usage_table(conn: &mut PooledConn) -> Result<()> {
    conn.query_drop(
        "CREATE TABLE IF NOT EXISTS insulin_usage (
            insulin_usage_id CHAR(36) PRIMARY KEY,
            insulin_assign_id VARCHAR(255) NOT NULL,
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
            batch_no VARCHAR(255) NOT NULL,
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
            insulin_item_name VARCHAR(255) NOT NULL,
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
        SELECT insulin_item_id, insulin_item_name, units, uom, created_at, notes, is_active
        FROM insulin_item
        WHERE insulin_item_id = :id AND is_active = 1
    "#;

    let result: Vec<InsulinItem> = conn.exec_map(
        query,
        params! { "id" => insulin_item_id },
        |(insulin_item_id, insulin_item_name, units, uom, created_at, notes, is_active): (String, String, f32, String, NaiveDateTime, Option<String>, i32)| {
            InsulinItem {
                insulin_item_id: Uuid::parse_str(&insulin_item_id)
                .unwrap_or_else(|_| Uuid::nil()),
                insulin_item_name: insulin_item_name,
                units: units,
                uom: uom,
                created_at: created_at,
                is_active: is_active,
                notes: notes,
            }
        },
    )?;

    Ok(result)
}

pub fn select_all_insulin_item(conn: &mut PooledConn) -> Result<Vec<InsulinItem>, Box<dyn Error>> {
    let query = r#"
        SELECT insulin_item_id, insulin_item_name, units, uom, created_at, notes, is_active
        FROM insulin_item
    "#;

    let result: Vec<InsulinItem> = conn.query_map(
        query,
        |(insulin_item_id, insulin_item_name, units,uom, created_at, notes, is_active): (String, String, f32, String, String, Option<String>, i32)| {
            InsulinItem {
                insulin_item_id: Uuid::parse_str(&insulin_item_id)
                .unwrap_or_else(|_| Uuid::nil()),
                insulin_item_name: insulin_item_name,
                units: units,
                uom: uom,
                notes: notes,
                created_at: NaiveDateTime::parse_from_str(&created_at, "%Y-%m-%d %H:%M:%S")
                .unwrap_or_else(|_| NaiveDateTime::from_timestamp_opt(0, 0).unwrap()),
                is_active: is_active
            }
        },
    )?;

    Ok(result)
}

pub fn insert_insulin_item(conn: &mut PooledConn, insulin_item: &InsulinItem) -> Result<DatabaseResult, Box<dyn Error>> {
    let query = r#"
        INSERT INTO insulin_item 
        (insulin_item_id, insulin_item_name, units, uom, created_at, notes, is_active)
        VALUES 
        (:id, :cat, :units, :uom, :created_at, :notes, :is_active)
    "#;

    let result = conn.exec_drop(
        query,
        params! {
            "id" => insulin_item.insulin_item_id.to_string(),
            "cat" => &insulin_item.insulin_item_name,
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

pub fn select_insulin_assign(conn: &mut PooledConn, insulin_assign_id: &str) -> Result<Vec<InsulinAssign>, Box<dyn Error>> {
    let query = r#"
        SELECT insulin_assign_id, insulin_item_id,  added_at, batch_no, notes, is_active
        FROM insulin_assign
        WHERE insulin_assign_id = :id AND is_active = 1
    "#;

    let result: Vec<InsulinAssign> = conn.exec_map(
        query,
        params! { "id" => insulin_assign_id },
        |(insulin_assign_id, insulin_item_id, added_at, batch_no, notes, is_active): (String, String, NaiveDateTime, String, Option<String>, i32)| {
            InsulinAssign {
                insulin_assign_id: Uuid::parse_str(&insulin_assign_id)
                .unwrap_or_else(|_| Uuid::nil()),
                insulin_item_id: Uuid::parse_str(&insulin_item_id)
                .unwrap_or_else(|_| Uuid::nil()),
                batch_no: batch_no,
                added_at: added_at,
                is_active: is_active,
                notes: notes,
            }
        },
    )?;

    Ok(result)
}

pub fn select_all_insulin_assign(conn: &mut PooledConn) -> Result<Vec<InsulinAssign>, Box<dyn Error>> {
    let query = r#"
        SELECT insulin_assign_id, insulin_item_id, batch_no, added_at, notes, is_active
        FROM insulin_assign
        WHERE is_active = 1
    "#;

    let result: Vec<InsulinAssign> = conn.query_map(
        query,
        |(insulin_assign_id, insulin_item_id, added_at, batch_no, notes, is_active): (String, String, String, String, Option<String>, i32)| {
            InsulinAssign {
                insulin_assign_id: Uuid::parse_str(&insulin_assign_id)
                .unwrap_or_else(|_| Uuid::nil()),
                insulin_item_id: Uuid::parse_str(&insulin_item_id)
                .unwrap_or_else(|_| Uuid::nil()),
                batch_no: batch_no,
                added_at: NaiveDateTime::parse_from_str(&added_at, "%Y-%m-%d %H:%M:%S")
                .unwrap_or_else(|_| NaiveDateTime::from_timestamp_opt(0, 0).unwrap()),
                is_active: is_active,
                notes: notes,
            }
        },
    )?;
    Ok(result)
}

pub fn select_all_insulin_assign_usage(conn: &mut PooledConn) -> Result<Vec<InsulinAssignUsage>, Box<dyn Error>> {
    let query = r#"
        SELECT a.insulin_assign_id, i.insulin_item_name, a.insulin_item_id, a.added_at, a.batch_no, a.notes, a.is_active, (i.units - IFNULL(SUM(u.units), 0)) as total_units
        FROM insulin_assign a
        inner join insulin_item i
        ON a.insulin_item_id = i.insulin_item_id
        left join insulin_usage u 
        ON a.insulin_assign_id = u.insulin_assign_id
        WHERE a.is_active = 1
        GROUP BY a.insulin_assign_id, a.insulin_item_id, a.added_at, a.notes, a.is_active
    "#;

    let result: Vec<InsulinAssignUsage> = conn.query_map(
        query,
        |(insulin_assign_id, insulin_item_name, insulin_item_id, added_at, batch_no, notes, is_active, total_units): (String, String, String, String,String, Option<String>,   i32, f32)| {
            InsulinAssignUsage {
                insulin_assign_id: Uuid::parse_str(&insulin_assign_id)
                .unwrap_or_else(|_| Uuid::nil()),
                insulin_item_id: Uuid::parse_str(&insulin_item_id)
                .unwrap_or_else(|_| Uuid::nil()),
                insulin_item_name: insulin_item_name,
                added_at: NaiveDateTime::parse_from_str(&added_at, "%Y-%m-%d %H:%M:%S")
                .unwrap_or_else(|_| NaiveDateTime::from_timestamp_opt(0, 0).unwrap()),
                is_active: is_active,
                batch_no: batch_no,
                notes: notes,
                total_units: total_units,
            }
        },
    )?;
    Ok(result)
}

pub fn insert_insulin_assign(conn: &mut PooledConn, insulin_assign: &InsulinAssign) -> Result<DatabaseResult, Box<dyn Error>> {
    let query = r#"
        INSERT INTO insulin_assign 
        (insulin_assign_id, insulin_item_id, batch_no, added_at, notes, is_active)
        VALUES 
        (:id, :item_id, :batch_no, :added_at, :notes, :is_active)
    "#;

    let result = conn.exec_drop(
        query,
        params! {
            "id" => insulin_assign.insulin_assign_id.to_string(),
            "item_id" => insulin_assign.insulin_item_id.to_string(),
            "added_at" => insulin_assign.added_at,
            "batch_no" => &insulin_assign.batch_no,
            "notes" => &insulin_assign.notes,
            "is_active" => insulin_assign.is_active,
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

pub fn delete_insulin_assign(conn: &mut PooledConn, insulin_assign_id: &str) -> Result<(), Box<dyn Error>> {
    conn.exec_drop(
        "UPDATE insulin_assign SET is_active = 0 WHERE insulin_assign_id = :id",
        params! { "id" => insulin_assign_id },
    )?;
    Ok(())
}

pub fn select_insulin_usage(conn: &mut PooledConn, insulin_assign_id: &str) -> Result<Vec<InsulinUsage>, Box<dyn Error>> {
    let query = r#"
        SELECT insulin_usage_id, insulin_assign_id, units, administered_at, notes, is_active
        FROM insulin_usage
        WHERE insulin_usage_id = :id AND is_active = 1
    "#;

    let result: Vec<InsulinUsage> = conn.exec_map(
        query,
        params! { "id" => insulin_assign_id },
        |(insulin_usage_id, insulin_assign_id, units, administered_at, notes, is_active): (String, String, f32, String, Option<String>, i32)| {
            InsulinUsage {
                insulin_usage_id: Uuid::parse_str(&insulin_usage_id)
                .unwrap_or_else(|_| Uuid::nil()),
                insulin_assign_id: Uuid::parse_str(&insulin_assign_id)
                .unwrap_or_else(|_| Uuid::nil()),
                units: units,
                administered_at: NaiveDateTime::parse_from_str(&administered_at, "%Y-%m-%d %H:%M:%S")
                .unwrap_or_else(|_| NaiveDateTime::from_timestamp_opt(0, 0).unwrap()),
                notes: notes,
                is_active: is_active,
            }
        },
    )?;

    Ok(result)
}

pub fn select_all_insulin_usage(conn: &mut PooledConn) -> Result<Vec<InsulinUsage>, Box<dyn Error>> {
    let query = r#"
        SELECT insulin_usage_id, insulin_assign_id, units, administered_at, notes, is_active
        FROM insulin_usage
        WHERE is_active = 1
    "#;

    let result: Vec<InsulinUsage> = conn.query_map(
        query,
        |(insulin_usage_id, insulin_assign_id, units, administered_at, notes, is_active): (String, String, f32, String, Option<String>, i32)| {
            InsulinUsage {
                insulin_usage_id: Uuid::parse_str(&insulin_usage_id)
                .unwrap_or_else(|_| Uuid::nil()),
                insulin_assign_id: Uuid::parse_str(&insulin_assign_id)
                .unwrap_or_else(|_| Uuid::nil()),
                units: units,
                administered_at: NaiveDateTime::parse_from_str(&administered_at, "%Y-%m-%d %H:%M:%S")
                .unwrap_or_else(|_| NaiveDateTime::from_timestamp_opt(0, 0).unwrap()),
                is_active: is_active,
                notes: notes,
            }
        },
    )?;
    Ok(result)
}

pub fn insert_insulin_usage(conn: &mut PooledConn, insulin_usage: &InsulinUsage) -> Result<DatabaseResult, Box<dyn Error>> {
    let query = r#"
        INSERT INTO insulin_usage 
        (insulin_usage_id, insulin_assign_id, units, administered_at, notes, is_active)
        VALUES 
        (:id, :item_id, :units, :administered_at, :notes, :is_active)
    "#;

    let result = conn.exec_drop(
        query,
        params! {
            "id" => insulin_usage.insulin_usage_id.to_string(),
            "item_id" => insulin_usage.insulin_assign_id.to_string(),
            "units" => insulin_usage.units,
            "administered_at" => insulin_usage.administered_at,
            "notes" => &insulin_usage.notes,
            "is_active" => insulin_usage.is_active,
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

pub fn delete_insulin_usage(conn: &mut PooledConn, insulin_usage_id: &str) -> Result<(), Box<dyn Error>> {
    conn.exec_drop(
        "UPDATE insulin_usage SET is_active = 0 WHERE insulin_usage_id = :id",
        params! { "id" => insulin_usage_id },
    )?;
    Ok(())
}