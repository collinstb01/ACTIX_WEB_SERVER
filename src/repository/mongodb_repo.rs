use std::{env, str::FromStr};
extern crate dotenv;
use dotenv::dotenv;
use futures::{io::Cursor, TryStreamExt};

use crate::models::{books_model::Book, user_model::User};
use mongodb::{
    bson::{doc, extjson::de::Error},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};

pub struct MongoRepo {
    col: Collection<User>,
    book_col: Collection<Book>,
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
        let book_col: Collection<Book> = db.collection("Book");

        MongoRepo { col, book_col }
    }

    // working
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

    // working
    pub async fn get_user(&self, id: &String) -> Result<User, Error> {
        let id = mongodb::bson::oid::ObjectId::parse_str(&id).expect("Error passing string");
        let filter = doc! {"_id": id};

        let user_data = self
            .col
            .find_one(filter, None)
            .await
            .expect("Error finding User");

        Ok(user_data.unwrap())
    }

    // working
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

    // working
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

    // working
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
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
    }

    pub async fn create_book(&self, data: Book) -> Result<InsertOneResult, Error> {
        let doc = Book {
            id: None,
            title: data.title,
            message: data.message,
            owner_id: data.owner_id,
        };
        let book_data = self
            .book_col
            .insert_one(doc, None)
            .await
            .expect("Unable to create book");
        Ok(book_data)
    }

    pub async fn get_books(&self, names: String) -> Result<Vec<Book>, Error> {
        let name_arr: Vec<&str> = names.split(",").collect();

        let filter = doc! {
            "title": {
                "$in": ["rich", "winners"]
            }
        };

        let mut data = self
            .book_col
            .find(filter, None)
            .await
            .ok()
            .expect("Unable to get book");

        let mut arr = Vec::new();

        while let Some(data) = data.try_next().await.expect("Unable to retrive data") {
            arr.push(data)
        }
        Ok(arr)
    }
}
