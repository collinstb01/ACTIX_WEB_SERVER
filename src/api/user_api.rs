use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use mongodb::bson::oid::ObjectId;
use regex::Regex;
use sha2::Sha256;
use std::collections::BTreeMap;

fn get_secret_key() -> Hmac<Sha256> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"keysec").unwrap();
    key
}

fn create_token(user_id: &String) -> String {
    let key = get_secret_key();
    let mut claims = BTreeMap::new();
    claims.insert("user_id", user_id);
    let token_str = claims.sign_with_key(&key).unwrap();

    token_str
}

fn verify_token(token_str: &String, user_id: &String) {
    let key = get_secret_key();
    // let token_str = "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJzb21lb25lIn0.5wwE1sBrs-vftww_BGIuTVDeHtc1Jsjo-fiHhDwR8m0";
    let claims: BTreeMap<String, String> = token_str.verify_with_key(&key).unwrap();

    assert_eq!(&claims["user_id"], user_id);
}

#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {
    // check the validity of the email
    let pattern = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";

    if !Regex::new(pattern).unwrap().is_match(&new_user.email) {
        return HttpResponse::InternalServerError().body("Not a valid email address");
    }

    // cehck the name length
    let user_name_len: Vec<&str> = new_user.name.split(" ").collect();

    if new_user.name.len() < 3 || user_name_len.len() < 2 {
        return HttpResponse::InternalServerError().body("Not a Name with two characters");
    }

    // check the password strenght
    if &new_user.password.len() < &8 {
        return HttpResponse::InternalServerError().body("Please enter a valid password, with Minimum eight characters, at least one letter and one number:");
    }

    // encrypt the password

    let data = User {
        id: None,
        name: new_user.name.to_owned(),
        email: new_user.email.to_owned(),
        password: new_user.password.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
        user_id: new_user.user_id.to_owned(),
    };

    let user_detail = db.create_user(data).await;
    // let token_str = create_token(user_detail);

    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/user/{id}")]
pub async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }

    let user_detail = db.get_user(&id).await;
    match user_detail {
        Ok(data) => return HttpResponse::Ok().json(data),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/user/{id}")]
pub async fn update_user(
    db: Data<MongoRepo>,
    path: Path<String>,
    user_data: Json<User>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }

    let new_user = User {
        id: Some(ObjectId::parse_str(&id).expect("Unable to parse string")),
        name: user_data.name.to_owned(),
        email: user_data.email.to_owned(),
        password: user_data.password.to_owned(),
        location: user_data.location.to_owned(),
        title: user_data.title.to_owned(),
        user_id: user_data.user_id.to_owned(),
    };

    let update_result = db.update_user(id.clone(), new_user).await;
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user(&id).await;
                return match updated_user_info {
                    Ok(user) => HttpResponse::Ok().json(user),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No user found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/user/{id}")]
pub async fn delete_user(db: Data<MongoRepo>, id: Path<String>) -> HttpResponse {
    let id = id.into_inner();

    let data = db.delete_user(id).await;
    match data {
        Ok(data) => return HttpResponse::Ok().json(data),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/users")]
pub async fn get_users(db: Data<MongoRepo>) -> HttpResponse {
    let data = db.get_users().await;

    match data {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(err) => HttpResponse::NotFound().body(err.to_string()),
    }
}
