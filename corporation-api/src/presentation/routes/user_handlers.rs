use actix_web::{HttpResponse, Responder, http::header::ContentType, web};
use serde::Serialize;

use crate::application::queries::read_user;
use crate::infrastructure::AppData;

#[derive(Serialize, Debug)]
struct Response {
    id: u32,
    name: String,
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    error: String,
}

pub async fn get(path: web::Path<u32>, app_data: web::Data<AppData>) -> impl Responder {
    let user_id: u32 = path.into_inner();

    let input = read_user::dto::ReadUserInput { user_id: user_id };
    let interactor = read_user::use_case::ReadUserInteractor::new(app_data.user_repository.clone());
    let result = interactor.execute(input).await;

    if result.is_ok() {
        let response = Response {
            id: user_id,
            name: result.unwrap().user_id.to_string(),
        };
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(web::Json(response))
    } else {
        let error = result.err().unwrap();

        match error {
            read_user::dto::ReadUserError::NotFound => {
                let error_response = ErrorResponse {
                    error: "User not found".to_string(),
                };
                HttpResponse::NotFound()
                    .content_type(ContentType::json())
                    .json(web::Json(error_response))
            }
            read_user::dto::ReadUserError::Unknown => {
                let error_response = ErrorResponse {
                    error: "Unknown error occurred".to_string(),
                };
                HttpResponse::InternalServerError()
                    .content_type(ContentType::json())
                    .json(web::Json(error_response))
            }
        }
    }
}

pub async fn post() -> impl Responder {
    HttpResponse::Ok().body("post user")
}
