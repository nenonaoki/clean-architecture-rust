use actix_web::{services, web};

mod user_handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(services![
        web::resource("/user/{user_id}")
            .route(web::get().to(user_handlers::get))
            .route(web::post().to(user_handlers::post)),
    ]);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
