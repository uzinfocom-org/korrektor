use actix_web::{web, Scope};

mod duplicate;
mod frequency;

pub fn scope() -> Scope {
    web::scope("/tools")
        .service(duplicate::sort)
        .service(frequency::sort)
}
