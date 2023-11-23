use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    error::Error as MongoError,  // Import the Error type from the mongodb crate
    results::InsertOneResult,
    Client, Collection,
};
use crate::models::user_model::User;
    
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

    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, MongoError> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        
        match self.col.insert_one(new_doc, None).await {
            Ok(user) => Ok(user),
            Err(err) => {
                eprintln!("Error creating user: {}", err);
                Err(err)
            }
        }
    }
}
