use actix_web::{web, Scope};

mod alphabetic;
mod number;
mod tokenize;

pub fn scope() -> Scope {
    web::scope("/tools")
        .service(alphabetic::sort)
        .service(number::sort)
        .service(tokenize::sort)
}
