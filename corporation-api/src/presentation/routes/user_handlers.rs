use actix_web::{HttpResponse, Responder, http::header::ContentType, web};
use serde::{Deserialize, Serialize};

use crate::application::queries::read_user;
use crate::domain::repositories::project::ProjectRepository;
use crate::domain::repositories::user::UserRepository;

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

pub async fn get(
    path: web::Path<u32>,
    query: web::Query<Info>,
    user_repository: web::Data<dyn UserRepository>,
    // project_repository: web::Data<dyn ProjectRepository>,
) -> impl Responder {
    let user_id: u32 = path.into_inner();
    // let query = query.into_inner();
    // let user_name = query.user_name;

    let input = read_user::dto::ReadUserInput { user_id: user_id };

    let user_repo = user_repository.into_inner();
    // let project_repo = project_repository.into_inner();
    let interactor = read_user::use_case::ReadUserInteractor::new(user_repo);
    let result = interactor.execute(input).await;

    // let result = read_user::use_case::interactor(input);

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
