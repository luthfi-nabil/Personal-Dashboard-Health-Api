use crate::models::insulin::{self, InsulinAssign, InsulinAssignUsage, InsulinItem, InsulinUsage};
use crate::models::responses::DatabaseResult;
use chrono::{DateTime, NaiveDateTime, Utc};
use mysql::prelude::*;
use mysql::{Error as MysqlError, PooledConn, Result, Value::Double, params};
use std::collections::HashMap;
use std::error::Error;
use uuid::Uuid;

pub fn create_insulin_usage_table(conn: &mut PooledConn) -> Result<()> {
    conn.query_drop(
        "CREATE TABLE IF NOT EXISTS insulin_usage (
            insulin_usage_id CHAR(36) PRIMARY KEY,
            insulin_assign_id VARCHAR(255) NOT NULL,
            units FLOAT NOT NULL,
            administered_at DATETIME NOT NULL,
            notes TEXT,
            is_active INT NOT NULL,
            created_by VARCHAR(255) NOT NULL
        )",
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
            is_active INT NOT NULL,
            created_by VARCHAR(255) NOT NULL
        )",
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
            is_active INT NOT NULL,
            created_by VARCHAR(255) NOT NULL
        )",
    )?;
    Ok(())
}

pub fn select_insulin_item(
    conn: &mut PooledConn,
    insulin_item: &InsulinItem,
) -> Result<Vec<InsulinItem>, Box<dyn Error>> {
    let mut query = String::from(
        r#"
        SELECT insulin_item_id, insulin_item_name, units, uom, created_at, notes, is_active, created_by
        FROM insulin_item
        WHERE is_active = 1
    "#,
    );

    let mut params: Vec<mysql::Value> = Vec::new();

    if insulin_item.insulin_item_id != Uuid::nil() {
        query.push_str(" AND insulin_item_id = ?");
        params.push(insulin_item.insulin_item_id.to_string().into());
    }

    if !insulin_item.created_by.is_empty() {
        query.push_str(" AND created_by = ?");
        params.push(insulin_item.created_by.clone().into());
    }

    let result: Vec<InsulinItem> = conn.exec_map(
        query,
        params,
        |(
            insulin_item_id,
            insulin_item_name,
            units,
            uom,
            created_at,
            notes,
            is_active,
            created_by,
        ): (
            String,
            String,
            f32,
            String,
            NaiveDateTime,
            Option<String>,
            i32,
            String,
        )| {
            InsulinItem {
                insulin_item_id: Uuid::parse_str(&insulin_item_id).unwrap_or_else(|_| Uuid::nil()),
                insulin_item_name: insulin_item_name,
                units: units,
                uom: uom,
                created_at: created_at,
                is_active: is_active,
                notes: notes,
                created_by: created_by,
            }
        },
    )?;

    Ok(result)
}

pub fn select_all_insulin_item(
    conn: &mut PooledConn,
    insulin_item: &InsulinItem,
) -> Result<Vec<InsulinItem>, Box<dyn Error>> {
    let mut query = String::from(
        r#"
        SELECT insulin_item_id, insulin_item_name, units, uom, created_at, notes, is_active, created_by
        FROM insulin_item
    "#,
    );

    let mut params: Vec<mysql::Value> = Vec::new();

    if insulin_item.is_active != 0 {
        query.push_str(" WHERE is_active = 1");
    } else {
        query.push_str(" WHERE is_active = 0");
    }

    if insulin_item.insulin_item_id != Uuid::nil() {
        query.push_str(" AND insulin_item_id = ?");
        params.push(insulin_item.insulin_item_id.to_string().into());
    }

    if !insulin_item.created_by.is_empty() {
        query.push_str(" AND created_by = ?");
        params.push(insulin_item.created_by.clone().into());
    }

    let result: Vec<InsulinItem> = conn.exec_map(
        query,
        params,
        |(
            insulin_item_id,
            insulin_item_name,
            units,
            uom,
            created_at,
            notes,
            is_active,
            created_by,
        ): (
            String,
            String,
            f32,
            String,
            NaiveDateTime,
            Option<String>,
            i32,
            String,
        )| {
            InsulinItem {
                insulin_item_id: Uuid::parse_str(&insulin_item_id).unwrap_or_else(|_| Uuid::nil()),
                insulin_item_name: insulin_item_name,
                units: units,
                uom: uom,
                notes: notes,
                created_at: created_at,
                is_active: is_active,
                created_by: created_by,
            }
        },
    )?;

    Ok(result)
}

pub fn insert_insulin_item(
    conn: &mut PooledConn,
    insulin_item: &InsulinItem,
) -> Result<DatabaseResult, Box<dyn Error>> {
    let query = r#"
        INSERT INTO insulin_item 
        (insulin_item_id, insulin_item_name, units, uom, created_at, notes, is_active, created_by)
        VALUES 
        (:id, :cat, :units, :uom, :created_at, :notes, :is_active, :created_by)
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
            "created_by" => &insulin_item.created_by,
        },
    );
    match result {
        Ok(_) => Ok(DatabaseResult::Inserted),

        Err(MysqlError::MySqlError(ref e)) => match e.code {
            1062u16 => Ok(DatabaseResult::Duplicate),
            _ => Err(Box::new(MysqlError::MySqlError(e.clone()))),
        },

        Err(e) => Err(Box::new(e)),
    }
}

pub fn delete_insulin_item(
    conn: &mut PooledConn,
    insulin_item_id: &str,
) -> Result<(), Box<dyn Error>> {
    conn.exec_drop(
        "UPDATE insulin_item SET is_active = 0 WHERE insulin_item_id = :id",
        params! { "id" => insulin_item_id },
    )?;
    Ok(())
}

pub fn select_insulin_assign(
    conn: &mut PooledConn,
    insulin_assign: &InsulinAssign,
) -> Result<Vec<InsulinAssign>, Box<dyn Error>> {
    let mut query = String::from(
        r#"
        SELECT insulin_assign_id, insulin_item_id,  added_at, batch_no, notes, is_active, created_by
        FROM insulin_assign
        WHERE is_active = 1
    "#,
    );
    let mut params: Vec<mysql::Value> = Vec::new();

    if insulin_assign.insulin_assign_id != Uuid::nil() {
        query.push_str(" AND insulin_assign_id = ?");
        params.push(insulin_assign.insulin_assign_id.to_string().into());
    }

    if insulin_assign.insulin_item_id != Uuid::nil() {
        query.push_str(" AND insulin_item_id = ?");
        params.push(insulin_assign.insulin_item_id.to_string().into());
    }

    if !insulin_assign.batch_no.is_empty() {
        query.push_str(" AND batch_no = ?");
        params.push(insulin_assign.batch_no.clone().into());
    }

    if !insulin_assign.created_by.is_empty() {
        query.push_str(" AND created_by = ?");
        params.push(insulin_assign.created_by.clone().into());
    }

    let result: Vec<InsulinAssign> =
        conn.exec_map(
            query,
            params,
            |(
                insulin_assign_id,
                insulin_item_id,
                added_at,
                batch_no,
                notes,
                is_active,
                created_by,
            ): (
                String,
                String,
                NaiveDateTime,
                String,
                Option<String>,
                i32,
                String,
            )| {
                InsulinAssign {
                    insulin_assign_id: Uuid::parse_str(&insulin_assign_id)
                        .unwrap_or_else(|_| Uuid::nil()),
                    insulin_item_id: Uuid::parse_str(&insulin_item_id)
                        .unwrap_or_else(|_| Uuid::nil()),
                    batch_no: batch_no,
                    added_at: added_at,
                    is_active: is_active,
                    notes: notes,
                    created_by: created_by,
                }
            },
        )?;

    Ok(result)
}

pub fn select_all_insulin_assign(
    conn: &mut PooledConn,
    insulin_assign: &InsulinAssign,
) -> Result<Vec<InsulinAssign>, Box<dyn Error>> {
    let mut query = String::from(
        r#"
        SELECT insulin_assign_id, insulin_item_id, batch_no,
            added_at, notes, is_active, created_by
        FROM insulin_assign
        WHERE is_active = 1
    "#,
    );

    let mut params: Vec<mysql::Value> = Vec::new();

    if insulin_assign.insulin_assign_id != Uuid::nil() {
        query.push_str(" AND a.insulin_assign_id = ?");
        params.push(insulin_assign.insulin_assign_id.to_string().into());
    }

    if insulin_assign.insulin_item_id != Uuid::nil() {
        query.push_str(" AND a.insulin_item_id = ?");
        params.push(insulin_assign.insulin_item_id.to_string().into());
    }

    if !insulin_assign.batch_no.is_empty() {
        query.push_str(" AND a.batch_no = ?");
        params.push(insulin_assign.batch_no.clone().into());
    }

    if !insulin_assign.created_by.is_empty() {
        query.push_str(" AND a.created_by = ?");
        params.push(insulin_assign.created_by.clone().into());
    }

    let result: Vec<InsulinAssign> =
        conn.exec_map(
            query,
            params,
            |(
                insulin_assign_id,
                insulin_item_id,
                added_at,
                batch_no,
                notes,
                is_active,
                created_by,
            ): (String, String, String, String, Option<String>, i32, String)| {
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
                    created_by: created_by,
                }
            },
        )?;
    Ok(result)
}

pub fn select_all_insulin_assign_usage(
    conn: &mut PooledConn,
    insulin_assign: &InsulinAssign,
) -> Result<Vec<InsulinAssignUsage>, Box<dyn Error>> {
    let mut query = String::from(
        r#"
        SELECT a.insulin_assign_id, i.insulin_item_name, a.insulin_item_id,
            a.added_at, a.batch_no, a.notes, a.is_active,
            (i.units - IFNULL(SUM(u.units), 0)) as total_units,
            max(u.administered_at) as last_used_at,
            a.created_by
        FROM insulin_assign a
        INNER JOIN insulin_item i
            ON a.insulin_item_id = i.insulin_item_id
        LEFT JOIN insulin_usage u
            ON a.insulin_assign_id = u.insulin_assign_id
        WHERE a.is_active = 1
    "#,
    );

    let mut params: Vec<mysql::Value> = Vec::new();

    if insulin_assign.insulin_assign_id != Uuid::nil() {
        query.push_str(" AND a.insulin_assign_id = ?");
        params.push(insulin_assign.insulin_assign_id.to_string().into());
    }

    if insulin_assign.insulin_item_id != Uuid::nil() {
        query.push_str(" AND a.insulin_item_id = ?");
        params.push(insulin_assign.insulin_item_id.to_string().into());
    }

    if !insulin_assign.batch_no.is_empty() {
        query.push_str(" AND a.batch_no = ?");
        params.push(insulin_assign.batch_no.clone().into());
    }

    if !insulin_assign.created_by.is_empty() {
        query.push_str(" AND a.created_by = ?");
        params.push(insulin_assign.created_by.clone().into());
    }

    query.push_str(
        " GROUP BY a.insulin_assign_id, a.insulin_item_id,
                a.added_at, a.notes, a.is_active",
    );
    let result: Vec<InsulinAssignUsage> = conn.exec_map(
        query,
        params,
        |(
            insulin_assign_id,
            insulin_item_name,
            insulin_item_id,
            added_at,
            batch_no,
            notes,
            is_active,
            total_units,
            last_used_at,
            created_by,
        ): (
            String,
            String,
            String,
            NaiveDateTime,
            String,
            Option<String>,
            i32,
            f64,
            Option<NaiveDateTime>,
            String,
        )| {
            InsulinAssignUsage {
                insulin_assign_id: Uuid::parse_str(&insulin_assign_id)
                    .unwrap_or_else(|_| Uuid::nil()),
                insulin_item_id: Uuid::parse_str(&insulin_item_id).unwrap_or_else(|_| Uuid::nil()),
                insulin_item_name: insulin_item_name,
                added_at: added_at,
                is_active: is_active,
                batch_no: batch_no,
                notes: notes,
                total_units: total_units as f32,
                last_used_at: last_used_at,
                created_by: created_by,
            }
        },
    )?;
    Ok(result)
}

pub fn insert_insulin_assign(
    conn: &mut PooledConn,
    insulin_assign: &InsulinAssign,
) -> Result<DatabaseResult, Box<dyn Error>> {
    let query = r#"
        INSERT INTO insulin_assign 
        (insulin_assign_id, insulin_item_id, batch_no, added_at, notes, is_active, created_by)
        VALUES 
        (:id, :item_id, :batch_no, :added_at, :notes, :is_active, :created_by)
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
            "created_by" => &insulin_assign.created_by,
        },
    );
    match result {
        Ok(_) => Ok(DatabaseResult::Inserted),

        Err(MysqlError::MySqlError(ref e)) => match e.code {
            1062u16 => Ok(DatabaseResult::Duplicate),
            _ => Err(Box::new(MysqlError::MySqlError(e.clone()))),
        },

        Err(e) => Err(Box::new(e)),
    }
}

pub fn delete_insulin_assign(
    conn: &mut PooledConn,
    insulin_assign: &InsulinAssign,
) -> Result<(), Box<dyn Error>> {
    conn.exec_drop(
        "UPDATE insulin_assign SET is_active = 0 WHERE insulin_assign_id = :id AND created_by = :created_by",
        params! { "id" => insulin_assign.insulin_assign_id, "created_by" => &insulin_assign.created_by },
    )?;
    Ok(())
}

pub fn select_insulin_usage(
    conn: &mut PooledConn,
    insulin_usage: &InsulinUsage,
) -> Result<Vec<InsulinUsage>, Box<dyn Error>> {
    let mut query = String::from(
        r#"
        SELECT insulin_usage_id, insulin_assign_id, units, administered_at, notes, is_active, created_by
        FROM insulin_usage
        WHERE is_active = 1
    "#,
    );

    let mut params: Vec<mysql::Value> = Vec::new();

    if insulin_usage.insulin_assign_id != Uuid::nil() {
        query.push_str(" AND insulin_assign_id = ?");
        params.push(insulin_usage.insulin_assign_id.to_string().into());
    }

    if !insulin_usage.created_by.is_empty() {
        query.push_str(" AND created_by = ?");
        params.push(insulin_usage.created_by.clone().into());
    }

    let result: Vec<InsulinUsage> = conn.exec_map(
        query,
        params,
        |(
            insulin_usage_id,
            insulin_assign_id,
            units,
            administered_at,
            notes,
            is_active,
            created_by,
        ): (
            String,
            String,
            f32,
            NaiveDateTime,
            Option<String>,
            i32,
            String,
        )| {
            InsulinUsage {
                insulin_usage_id: Uuid::parse_str(&insulin_usage_id)
                    .unwrap_or_else(|_| Uuid::nil()),
                insulin_assign_id: Uuid::parse_str(&insulin_assign_id)
                    .unwrap_or_else(|_| Uuid::nil()),
                units: units,
                administered_at: administered_at,
                notes: notes,
                is_active: is_active,
                created_by: created_by,
            }
        },
    )?;

    Ok(result)
}

pub fn select_all_insulin_usage(
    conn: &mut PooledConn,
    insulin_usage: &InsulinUsage,
) -> Result<Vec<InsulinUsage>, Box<dyn Error>> {
    let mut query = String::from(
        r#"
        SELECT insulin_usage_id, insulin_assign_id, units, administered_at, notes, is_active, created_by
        FROM insulin_usage
        WHERE is_active = 1
    "#,
    );

    let mut params: Vec<mysql::Value> = Vec::new();

    if insulin_usage.insulin_assign_id != Uuid::nil() {
        query.push_str(" AND insulin_assign_id = ?");
        params.push(insulin_usage.insulin_assign_id.to_string().into());
    }

    if !insulin_usage.created_by.is_empty() {
        query.push_str(" AND created_by = ?");
        params.push(insulin_usage.created_by.clone().into());
    }

    let result: Vec<InsulinUsage> = conn.exec_map(
        query,
        params,
        |(
            insulin_usage_id,
            insulin_assign_id,
            units,
            administered_at,
            notes,
            is_active,
            created_by,
        ): (
            String,
            String,
            f32,
            NaiveDateTime,
            Option<String>,
            i32,
            String,
        )| {
            InsulinUsage {
                insulin_usage_id: Uuid::parse_str(&insulin_usage_id)
                    .unwrap_or_else(|_| Uuid::nil()),
                insulin_assign_id: Uuid::parse_str(&insulin_assign_id)
                    .unwrap_or_else(|_| Uuid::nil()),
                units: units,
                administered_at: administered_at,
                is_active: is_active,
                notes: notes,
                created_by: created_by,
            }
        },
    )?;
    Ok(result)
}

pub fn insert_insulin_usage(
    conn: &mut PooledConn,
    insulin_usage: &InsulinUsage,
) -> Result<DatabaseResult, Box<dyn Error>> {
    let query = r#"
        INSERT INTO insulin_usage 
        (insulin_usage_id, insulin_assign_id, units, administered_at, notes, is_active, created_by)
        VALUES 
        (:id, :item_id, :units, :administered_at, :notes, :is_active, :created_by)
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
            "created_by" => &insulin_usage.created_by,
        },
    );
    match result {
        Ok(_) => Ok(DatabaseResult::Inserted),

        Err(MysqlError::MySqlError(ref e)) => match e.code {
            1062u16 => Ok(DatabaseResult::Duplicate),
            _ => Err(Box::new(MysqlError::MySqlError(e.clone()))),
        },

        Err(e) => Err(Box::new(e)),
    }
}

pub fn delete_insulin_usage(
    conn: &mut PooledConn,
    insulin_usage: &InsulinUsage,
) -> Result<(), Box<dyn Error>> {
    conn.exec_drop(
        "UPDATE insulin_usage SET is_active = 0 WHERE insulin_usage_id = :id AND created_by = :created_by",
        params! { "id" => insulin_usage.insulin_usage_id, "created_by" => &insulin_usage.created_by },
    )?;
    Ok(())
}
