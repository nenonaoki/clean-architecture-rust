use actix_web::{HttpResponse, Responder, http::header::ContentType, web};
use serde::Serialize;

use crate::application::queries::read_user::{
    dto::ReadUserError, dto::ReadUserInput, dto::ReadUserOutput, use_case::ReadUserInteractor,
};
use crate::infrastructure::AppData;
use crate::infrastructure::persistences::user::UserRepositoryImpl;

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
    // Parse Request
    let user_id: u32 = path.into_inner();

    // Execute UseCase
    let input = ReadUserInput { user_id: user_id };
    let user_repository = app_data
        .container
        .resolve::<UserRepositoryImpl>("UserRepository")
        .unwrap();
    let interactor = ReadUserInteractor::new(user_repository);
    let result = interactor.execute(input).await;

    // Handle Response
    if result.is_ok() {
        let output: ReadUserOutput = result.unwrap();
        let response = Response {
            id: user_id,
            name: output.user_id.to_string(),
        };
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(web::Json(response))
    } else {
        let error: ReadUserError = result.err().unwrap();

        match error {
            ReadUserError::NotFound => {
                let error_response = ErrorResponse {
                    error: "User not found".to_string(),
                };
                HttpResponse::NotFound()
                    .content_type(ContentType::json())
                    .json(web::Json(error_response))
            }
            ReadUserError::Unknown => {
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
