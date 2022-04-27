use actix_web::{get, HttpResponse, post, Responder, web};
use bcrypt::{DEFAULT_COST, hash};
use bson::doc;
use log::info;

use crate::AppState;
use crate::model::Status;
use crate::model::user::User;

#[get("/users")]
pub async fn find_all_users() -> impl Responder {
    HttpResponse::Ok().body("User!")
}

#[post("/users")]
pub async fn create_user(data: web::Data<AppState>, body: web::Json<User>) -> impl Responder {
    let collection = data.db.collection("users");

    let user = collection.count_documents(doc! {
        "email": &body.email
    }, None).await;
    match user {
        Ok(count) => {
            if count > 0 {
                return HttpResponse::Ok().json(Status {
                    status: "User already exists".to_string(),
                });
            }
        }
        Err(_) => (),
    }

    let hashed = hash(&body.password, DEFAULT_COST).unwrap();
    let result = collection.insert_one(doc! {
        "name": &body.name,
        "email": &body.email,
        "password": &hashed,
    }, None).await;

    return match result {
        Ok(db_result) => {
            if let Some(new_id) = db_result.inserted_id.as_object_id() {
                info!("New document inserted with id {}", new_id);
            }
            let response = Status {
                status: "Successful".to_string(),
            };
            HttpResponse::Created().json(response)
        }
        Err(err) =>
            {
                info!("Failed! {}", err.to_string());
                HttpResponse::InternalServerError().finish()
            }
    };
}
