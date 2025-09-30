use actix_web::web::{self, ServiceConfig};

use crate::handlers::{
    get_course_detail, get_courses_for_teacher, health_check_handler, new_course,
};

pub fn general_routes(config: &mut ServiceConfig) {
    config.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(config: &mut ServiceConfig) {
    config.service(
        web::scope("courses")
            .route("/", web::post().to(new_course))
            .route("/{user_id}", web::get().to(get_courses_for_teacher))
            .route("/{user_id}/{course_id}", web::get().to(get_course_detail)),
    );
}
