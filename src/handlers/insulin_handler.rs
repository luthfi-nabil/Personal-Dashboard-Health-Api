use crate::helper::connection::establish_connection_v2;
use crate::models::insulin::{InsulinAssign, InsulinItem, InsulinUsage};
use crate::models::responses::{DatabaseResult, Response};
use crate::repository::insulin_repository::{
    delete_insulin_assign, delete_insulin_item, delete_insulin_usage, insert_insulin_assign,
    insert_insulin_item, insert_insulin_usage, select_all_insulin_assign_usage,
    select_all_insulin_item, select_all_insulin_usage, select_insulin_assign, select_insulin_item,
};
use crate::route_middleware::get_user::CreatedBy;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use chrono::{DateTime, Local, Utc};
use uuid::Uuid;

pub async fn get_all_insulin_items_api(req: HttpRequest) -> HttpResponse {
    let mut conn = establish_connection_v2().expect("Failed to connect to database");
    let created_by = req.extensions().get::<CreatedBy>().unwrap().0.clone();
    let insulin_item = InsulinItem {
        insulin_item_id: Uuid::nil(),
        insulin_item_name: "".to_string(),
        uom: "".to_string(),
        units: 0.0,
        created_at: Local::now().naive_local(),
        notes: None,
        is_active: 1,
        created_by: created_by,
    };
    let _result = select_all_insulin_item(&mut conn, &insulin_item);

    match _result {
        Ok(sources) => {
            let response = Response {
                status: "Success".to_string(),
                code: crate::helper::response_code::RESPONSE_CODE_DATA_RETRIEVAL_SUCCESS,
                message: "Success get sources".to_string(),
                description: "".to_string(),
                data: Some(serde_json::to_value(sources).unwrap()),
                success: true,
            };
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            let response = Response {
                status: "Error".to_string(),
                code: crate::helper::response_code::ERROR_CODE_DATA_RETRIEVAL_FAILED,
                message: "Failed to retrieve sources".to_string(),
                description: err.to_string(),
                data: None,
                success: false,
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

pub async fn post_insulin_items_api(
    req: HttpRequest,
    insulin: web::Json<InsulinItem>,
) -> HttpResponse {
    let mut conn = establish_connection_v2().expect("Failed to connect to database");
    let created_by = req.extensions_mut().get::<CreatedBy>().unwrap().0.clone();
    println!("Creating insulin item for user: {}", created_by);
    let new_insulin = InsulinItem {
        insulin_item_id: Uuid::new_v4(),
        insulin_item_name: insulin.insulin_item_name.clone(),
        uom: insulin.uom.clone(),
        units: insulin.units,
        created_at: Local::now().naive_local(),
        notes: insulin.notes.clone(),
        created_by: created_by.clone(),
        is_active: 1,
    };

    let mut response = Response {
        status: "Success".to_string(),
        code: crate::helper::response_code::RESPONSE_CODE_DATA_INSERTION_SUCCESS,
        message: "Insulin type created successfully".to_string(),
        description: "".to_string(),
        data: None,
        success: true,
    };
    let _result = insert_insulin_item(&mut conn, &new_insulin);

    if _result.is_err() {
        response = Response {
            status: "Error".to_string(),
            message: "Failed to create insulin type".to_string(),
            code: crate::helper::response_code::ERROR_CODE_DATA_INSERTION_FAILED,
            description: _result.err().unwrap().to_string(),
            data: None,
            success: false,
        };
    } else {
        response.data = Some(serde_json::to_value(new_insulin).unwrap());
    }
    if response.code == crate::helper::response_code::RESPONSE_CODE_DATA_INSERTION_SUCCESS {
        HttpResponse::Created().json(response)
    } else {
        response.success = false;
        HttpResponse::BadRequest().json(response)
    }
}

pub async fn delete_insulin_items_api(req: HttpRequest) -> HttpResponse {
    let mut conn = establish_connection_v2().expect("Failed to connect to database");
    let insulin_item_id = req.extensions().get::<CreatedBy>().unwrap().0.clone();
    let result = delete_insulin_item(&mut conn, &insulin_item_id);

    match result {
        Ok(_) => {
            let response = Response {
                status: "Success".to_string(),
                code: crate::helper::response_code::RESPONSE_CODE_DATA_RETRIEVAL_SUCCESS,
                message: "Success delete insulin item".to_string(),
                description: "".to_string(),
                data: None,
                success: true,
            };
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            let response = Response {
                status: "Error".to_string(),
                code: crate::helper::response_code::ERROR_CODE_DATA_RETRIEVAL_FAILED,
                message: "Failed to delete insulin item".to_string(),
                description: err.to_string(),
                data: None,
                success: false,
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

pub async fn get_all_insulin_assign_usage_api(req: HttpRequest) -> HttpResponse {
    let mut conn = establish_connection_v2().expect("Failed to connect to database");
    let created_by = req.extensions().get::<CreatedBy>().unwrap().0.clone();
    let insulin_assign_usage = InsulinAssign {
        insulin_assign_id: Uuid::nil(),
        insulin_item_id: Uuid::nil(),
        batch_no: "".to_string(),
        added_at: Local::now().naive_local(),
        is_active: 1,
        created_by: created_by,
        notes: None,
    };
    match select_all_insulin_assign_usage(&mut conn, &insulin_assign_usage) {
        Ok(categories) => {
            let response = Response {
                status: "Success".to_string(),
                code: crate::helper::response_code::RESPONSE_CODE_DATA_RETRIEVAL_SUCCESS,
                message: "Success get insulin assign usage".to_string(),
                description: "".to_string(),
                data: Some(serde_json::to_value(categories).unwrap()),
                success: true,
            };
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            let response = Response {
                status: "Error".to_string(),
                code: crate::helper::response_code::ERROR_CODE_DATA_RETRIEVAL_FAILED,
                message: "Failed to retrieve earning categories".to_string(),
                description: err.to_string(),
                data: None,
                success: false,
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

pub async fn post_insulin_assign_api(
    req: HttpRequest,
    insulin_assign: web::Json<InsulinAssign>,
) -> HttpResponse {
    let mut conn = establish_connection_v2().expect("Failed to connect to database");
    let created_by = req.extensions().get::<CreatedBy>().unwrap().0.clone();
    let new_insulin_assign = InsulinAssign {
        insulin_assign_id: Uuid::new_v4(),
        insulin_item_id: insulin_assign.insulin_item_id,
        batch_no: insulin_assign.batch_no.clone(),
        added_at: Local::now().naive_local(),
        is_active: 1,
        created_by: created_by.clone(),
        notes: insulin_assign.notes.clone(),
    };

    let insulin_item = InsulinItem {
        insulin_item_id: new_insulin_assign.insulin_item_id,
        insulin_item_name: "".to_string(),
        uom: "".to_string(),
        units: 0.0,
        created_at: Local::now().naive_local(),
        notes: None,
        is_active: 1,
        created_by: created_by.clone(),
    };
    let _check_insulin_item = select_insulin_item(&mut conn, &insulin_item);

    let mut response = Response {
        status: "Success".to_string(),
        code: crate::helper::response_code::RESPONSE_CODE_DATA_INSERTION_SUCCESS,
        message: "Insulin assign created successfully".to_string(),
        description: "".to_string(),
        data: None,
        success: true,
    };

    if _check_insulin_item.is_ok() && _check_insulin_item.as_ref().unwrap().len() > 0 {
        let _result = insert_insulin_assign(&mut conn, &new_insulin_assign);

        if _result.is_err() {
            response = Response {
                status: "Error".to_string(),
                message: "Failed to create insulin assign".to_string(),
                code: crate::helper::response_code::ERROR_CODE_DATA_INSERTION_FAILED,
                description: _result.err().unwrap().to_string(),
                data: None,
                success: false,
            };
        } else {
            response.data = Some(serde_json::to_value(new_insulin_assign).unwrap());
        }
    } else {
        if _check_insulin_item.is_err() {
            response = Response {
                status: "Error".to_string(),
                code: crate::helper::response_code::ERROR_CODE_DATA_INSERTION_FAILED,
                message: "Failed to create insulin item".to_string(),
                description: _check_insulin_item.err().unwrap().to_string(),
                data: None,
                success: false,
            };
        } else if _check_insulin_item.as_ref().unwrap().len() == 0 {
            response = Response {
                status: "Error".to_string(),
                code: crate::helper::response_code::ERROR_CODE_DATA_INSERTION_FAILED,
                message: "Insulin item not found".to_string(),
                description: "Please create the insulin item first.".to_string(),
                data: None,
                success: false,
            };
        }
    }
    if response.code == crate::helper::response_code::RESPONSE_CODE_DATA_INSERTION_SUCCESS {
        HttpResponse::Created().json(response)
    } else {
        response.success = false;
        HttpResponse::BadRequest().json(response)
    }
}

pub async fn delete_insulin_assign_api(
    req: HttpRequest,
    path: web::Path<(String)>,
) -> HttpResponse {
    let mut conn = establish_connection_v2().expect("Failed to connect to database");
    let created_by = req.extensions().get::<CreatedBy>().unwrap().0.clone();
    let insulin_assign_id = path.into_inner();
    let insulin_assign = InsulinAssign {
        insulin_assign_id: Uuid::parse_str(&insulin_assign_id).unwrap(),
        insulin_item_id: Uuid::nil(),
        batch_no: "".to_string(),
        added_at: Local::now().naive_local(),
        is_active: 1,
        created_by: created_by,
        notes: None,
    };
    let result = delete_insulin_assign(&mut conn, &insulin_assign);

    match result {
        Ok(_) => {
            let response = Response {
                status: "Success".to_string(),
                code: crate::helper::response_code::RESPONSE_CODE_DATA_RETRIEVAL_SUCCESS,
                message: "Success delete insulin assign".to_string(),
                description: "".to_string(),
                data: None,
                success: true,
            };
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            let response = Response {
                status: "Error".to_string(),
                code: crate::helper::response_code::ERROR_CODE_DATA_RETRIEVAL_FAILED,
                message: "Failed to delete insulin assign".to_string(),
                description: err.to_string(),
                data: None,
                success: false,
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

pub async fn get_all_insulin_usage_api(req: HttpRequest) -> HttpResponse {
    let mut conn = establish_connection_v2().expect("Failed to connect to database");
    let created_by = req.extensions().get::<CreatedBy>().unwrap().0.clone();
    let insulin_usage = InsulinUsage {
        insulin_usage_id: Uuid::nil(),
        insulin_assign_id: Uuid::nil(),
        units: 0.0,
        administered_at: Local::now().naive_local(),
        is_active: 1,
        created_by: created_by,
        notes: None,
    };
    let _result = select_all_insulin_usage(&mut conn, &insulin_usage);

    match _result {
        Ok(sources) => {
            let response = Response {
                status: "Success".to_string(),
                code: crate::helper::response_code::RESPONSE_CODE_DATA_RETRIEVAL_SUCCESS,
                message: "Success get sources".to_string(),
                description: "".to_string(),
                data: Some(serde_json::to_value(sources).unwrap()),
                success: true,
            };
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            let response = Response {
                status: "Error".to_string(),
                code: crate::helper::response_code::ERROR_CODE_DATA_RETRIEVAL_FAILED,
                message: "Failed to retrieve sources".to_string(),
                description: err.to_string(),
                data: None,
                success: false,
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}

pub async fn post_insulin_usage_api(
    req: HttpRequest,
    insulin: web::Json<InsulinUsage>,
) -> HttpResponse {
    let mut conn = establish_connection_v2().expect("Failed to connect to database");
    let created_by = req.extensions().get::<CreatedBy>().unwrap().0.clone();
    let new_insulin = InsulinUsage {
        insulin_usage_id: Uuid::new_v4(),
        insulin_assign_id: insulin.insulin_assign_id.clone(),
        units: insulin.units,
        administered_at: Local::now().naive_local(),
        notes: insulin.notes.clone(),
        created_by: created_by.clone(),
        is_active: 1,
    };

    let insulin_assign = InsulinAssign {
        insulin_assign_id: new_insulin.insulin_assign_id,
        insulin_item_id: Uuid::nil(),
        batch_no: "".to_string(),
        added_at: Local::now().naive_local(),
        is_active: 1,
        created_by: created_by.clone(),
        notes: None,
    };

    let _check_insulin_item = select_insulin_assign(&mut conn, &insulin_assign);

    let mut response = Response {
        status: "Success".to_string(),
        code: crate::helper::response_code::RESPONSE_CODE_DATA_INSERTION_SUCCESS,
        message: "Insulin usage created successfully".to_string(),
        description: "".to_string(),
        data: None,
        success: true,
    };

    if _check_insulin_item.is_ok() && _check_insulin_item.as_ref().unwrap().len() > 0 {
        let _result = insert_insulin_usage(&mut conn, &new_insulin);

        if _result.is_err() {
            response = Response {
                status: "Error".to_string(),
                message: "Failed to create insulin usage".to_string(),
                code: crate::helper::response_code::ERROR_CODE_DATA_INSERTION_FAILED,
                description: _result.err().unwrap().to_string(),
                data: None,
                success: false,
            };
        } else {
            response.data = Some(serde_json::to_value(new_insulin).unwrap());
        }
    } else {
        if _check_insulin_item.is_err() {
            response = Response {
                status: "Error".to_string(),
                code: crate::helper::response_code::ERROR_CODE_DATA_INSERTION_FAILED,
                message: "Failed to create insulin usage".to_string(),
                description: _check_insulin_item.err().unwrap().to_string(),
                data: None,
                success: false,
            };
        } else if _check_insulin_item.as_ref().unwrap().len() == 0 {
            response = Response {
                status: "Error".to_string(),
                code: crate::helper::response_code::ERROR_CODE_DATA_INSERTION_FAILED,
                message: "Insulin assign not found".to_string(),
                description: "Please create the insulin assign first.".to_string(),
                data: None,
                success: false,
            };
        }
    }
    if response.code == crate::helper::response_code::RESPONSE_CODE_DATA_INSERTION_SUCCESS {
        HttpResponse::Created().json(response)
    } else {
        response.success = false;
        HttpResponse::BadRequest().json(response)
    }
}

pub async fn delete_insulin_usage_api(req: HttpRequest, path: web::Path<String>) -> HttpResponse {
    let mut conn = establish_connection_v2().expect("Failed to connect to database");
    let created_by = req.extensions().get::<CreatedBy>().unwrap().0.clone();
    let insulin_usage_id = path.into_inner();
    let insulin_usage = InsulinUsage {
        insulin_usage_id: Uuid::parse_str(&insulin_usage_id).unwrap(),
        insulin_assign_id: Uuid::nil(),
        units: 0.0,
        administered_at: Local::now().naive_local(),
        is_active: 1,
        created_by: created_by,
        notes: None,
    };
    let result = delete_insulin_usage(&mut conn, &insulin_usage);

    match result {
        Ok(_) => {
            let response = Response {
                status: "Success".to_string(),
                code: crate::helper::response_code::RESPONSE_CODE_DATA_RETRIEVAL_SUCCESS,
                message: "Success delete insulin item".to_string(),
                description: "".to_string(),
                data: None,
                success: true,
            };
            HttpResponse::Ok().json(response)
        }
        Err(err) => {
            let response = Response {
                status: "Error".to_string(),
                code: crate::helper::response_code::ERROR_CODE_DATA_RETRIEVAL_FAILED,
                message: "Failed to delete insulin item".to_string(),
                description: err.to_string(),
                data: None,
                success: false,
            };
            HttpResponse::InternalServerError().json(response)
        }
    }
}
