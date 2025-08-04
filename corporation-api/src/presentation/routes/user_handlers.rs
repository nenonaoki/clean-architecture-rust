use actix_web::{HttpResponse, Responder, http::header::ContentType, web};
use serde::{Deserialize, Serialize};

use crate::application::queries::read_user;

#[derive(Deserialize, Debug)]
pub struct Info {
    user_name: String,
}

#[derive(Serialize, Debug)]
struct Response {
    id: u32,
    name: String,
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    error: String,
}

pub async fn get(path: web::Path<u32>, query: web::Query<Info>) -> impl Responder {
    let user_id = path.into_inner();
    let query = query.into_inner();
    // println!("path: {:?}", user_id);
    // println!("query: {:?}", query);
    let user_name = query.user_name;
    let input: read_user::dto::Input = read_user::dto::Input {
        user_name: user_name,
    };
    let result = read_user::use_case::interactor(input);

    if result.is_ok() {
        let response = Response {
            id: user_id,
            name: result.unwrap().user_name,
        };
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(web::Json(response))
    } else {
        let error = result.err().unwrap();

        match error.status {
            read_user::dto::ErrorStatus::Unknown => {
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
