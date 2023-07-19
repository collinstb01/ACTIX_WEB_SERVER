use std::{env, str::FromStr};
extern crate dotenv;
use actix_web::{dev::Path, HttpResponse};
use dotenv::dotenv;
use futures::TryStreamExt;

use crate::models::user_model::User;
use mongodb::{
    bson::{doc, extjson::de::Error},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");
        MongoRepo { col }
    }

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            email: new_user.email,
            password: new_user.password,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .await
            .expect("Error creating user");
        Ok(user)
    }

    pub async fn get_user(&self, id: &String) -> Result<User, Error> {
        let id = mongodb::bson::oid::ObjectId::from_str(&id).expect("Error passing string");
        let filter = doc! {"_id": id};

        let user_data = self
            .col
            .find_one(filter, None)
            .await
            .expect("Error finding User");

        Ok(user_data.unwrap())
    }

    pub async fn update_user(&self, id: String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = mongodb::bson::oid::ObjectId::from_str(&id)
            .expect("Unable to convert string to object id");
        let filter = doc! {"_id": obj_id};

        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "name": new_user.name,
                    "location": new_user.location,
                    "title": new_user.title
                },
        };

        let data = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .expect("Unable to update user");
        Ok(data)
    }

    pub async fn delete_user(&self, id: String) -> Result<DeleteResult, Error> {
        let id = mongodb::bson::oid::ObjectId::from_str(&id).expect("Unable to pass string");
        let filter = doc! {"_id": id};

        let data = self
            .col
            .delete_one(filter, None)
            .await
            .expect("Unable to delete document");

        Ok(data)
    }

    pub async fn get_users(&self) -> Result<Vec<User>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .expect("Unable to get all users");

        let mut users = Vec::new();

        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
    }
}
