
use actix_web::{HttpResponse, Result, error::ResponseError, get, post, web::Json};
use serde::{Serialize, Deserialize};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
struct MyError {
    value: &'static str
}

impl ResponseError for MyError {}

#[derive(Deserialize, Default)]
struct MessageIn {
    pub string_field: Option<String>,
}

#[derive(Serialize, Default)]
struct MessageOut {
    pub field1: Option<i32>,
    pub string_field: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
struct MessageInOut {
    pub field1: Option<i32>,
    pub string_field: Option<String>,
}

#[get("/")]
async fn index() -> Result<String> {
    Ok("Hello".into())
}


#[post("/json")]
async fn json_in(m: Json<MessageIn>) -> Result<String> {
    if let Some(f2) = &m.string_field {
        Ok(f2.clone())
    } else {
        Err(MyError{
            value: "Invalid JSON"
        }.into())
    }
}


#[get("/json")]
async fn json_out() -> HttpResponse {
    HttpResponse::Ok().json(MessageOut {
            string_field: Some("something".into()),
            ..Default::default()
        }
    )

}


#[get("/json2")]
async fn json_out2() -> Result<Json<MessageOut>> {
    Ok(Json(MessageOut {
            string_field: Some("something".into()),
            ..Default::default()
        }))
}



#[post("/json_inout")]
async fn json_inout(m: Json<MessageInOut>) -> Result<Json<MessageInOut>> {
    Ok(m)
}




pub fn create_service() -> actix_web::Scope {
    actix_web::Scope::new("")
        .service(index)
        .service(json_in)
        .service(json_out)
        .service(json_out2)
        .service(json_inout)

}