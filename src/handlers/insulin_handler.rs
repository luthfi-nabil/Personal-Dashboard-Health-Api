use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc, Local};
use uuid::Uuid;
use crate::models::insulin::{InsulinUsage, InsulinItem, InsulinAssign};
use crate::models::responses::{Response, DatabaseResult};
use crate::helper::connection::{establish_connection_v2};
use crate::repository::insulin_repository::{
    select_insulin_item,
    select_all_insulin_item,
    insert_insulin_item,
    delete_insulin_item
};

pub async fn get_all_insulin_items_api(query: web::Query<InsulinItem>) -> HttpResponse {
    let mut conn = establish_connection_v2().expect("Failed to connect to database");
    let _result = select_all_insulin_item(&mut conn);

    match _result {
        Ok(sources) => {
            let response = Response {
                status: "Success".to_string(),
                code: crate::helper::response_code::RESPONSE_CODE_DATA_RETRIEVAL_SUCCESS,
                message: "Success get sources".to_string(),
                description: "".to_string(),
                data: Some(serde_json::to_value(sources).unwrap()),
            };
            HttpResponse::Ok().json(response)
        },
        Err(err) => {
            let response = Response {
                status: "Error".to_string(),
                code: crate::helper::response_code::ERROR_CODE_DATA_RETRIEVAL_FAILED,
                message: "Failed to retrieve sources".to_string(),
                description: err.to_string(),
                data: None,
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

pub async fn post_insulin_items_api(insulin: web::Json<InsulinItem>) -> HttpResponse {
    let mut conn = establish_connection_v2().expect("Failed to connect to database");
    let new_insulin = InsulinItem {
        insulin_item_id: Uuid::new_v4(),
        insulin_item: insulin.insulin_item.clone(),
        uom: insulin.uom.clone(),
        units: insulin.units,
        created_at: Local::now().naive_local(),
        notes: insulin.notes.clone(),
        is_active: 1
    };

    let mut response = Response {
        status: "Success".to_string(),
        code: crate::helper::response_code::RESPONSE_CODE_DATA_INSERTION_SUCCESS,
        message: "Insulin type created successfully".to_string(),
        description: "".to_string(),
        data: None,
    };
    let _result  = insert_insulin_item(&mut conn, &new_insulin);
        
    if _result.is_err() {
        response = Response {
            status: "Error".to_string(),
            message: "Failed to create insulin type".to_string(),
            code: crate::helper::response_code::ERROR_CODE_DATA_INSERTION_FAILED,
            description: _result.err().unwrap().to_string(),
            data: None,
        };
    }else{
        response.data = Some(serde_json::to_value(new_insulin).unwrap());
    }
    if response.code == crate::helper::response_code::RESPONSE_CODE_DATA_INSERTION_SUCCESS {
        HttpResponse::Created().json(response)
    }else{
        HttpResponse::BadRequest().json(response)
    }
    
}