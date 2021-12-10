use mongodb::bson::{doc, document::Document, oid::ObjectId};
use mongodb::{sync::Client, sync::Database, options::ClientOptions };
use chrono::{DateTime, offset::Utc, Datelike};
use serde::{Deserialize, Serialize};

pub struct MongoDB {
    client: mongodb::sync::Client,
    db: Database, 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Customer {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub message: String,
    pub ordered_service: String,
    pub examined_doctor: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub order_datetime: DateTime<Utc>,
}

impl MongoDB {
    pub fn client(&self) -> Client {
        self.client.clone()
    }

    pub fn init() -> MongoDB {
        let mut client_options = ClientOptions::parse("mongodb+srv://admin:admin@cluster0.eqn0h.mongodb.net/vetclinic?retryWrites=true&w=majority").expect("Failed to parse options!");
        client_options.app_name = Some("petto".to_string());

        let client = Client::with_options(client_options)
            .expect("Failed to initialize database!");


        let db = client.database("vetclinic"); MongoDB { client, db }
    }

    pub fn get_customers(&self) -> Vec<Customer> {
       let mut cursor = self.db
           .collection::<Customer>("customers")
           .find(None, None).expect("Not found");
       let mut result: Vec<Customer> = Vec::new(); 
       while let Some(obj) = cursor.next() {
           let mut customer = obj.expect("not found customer");
           let doctor = DoctorRepository::find_by_id(&customer.examined_doctor, &self).expect("Not found doctor");
           customer.examined_doctor = doctor.get("name").expect("not found").as_str().unwrap().to_string();
           let service = ServiceRepository::find_by_id(&customer.ordered_service, &self).expect("Not found service");
           customer.ordered_service = service.get("name").expect("not found service").as_str().unwrap().to_string(); 
           result.push(customer);
       }
       result
    } 
}

pub trait MongoEntity {
    fn collection() -> String;
}

pub trait MongoRepository {
    type Entity: MongoEntity;

    fn find_by_id(id: &str, db: &MongoDB) -> Option<Document> {
        let oid = ObjectId::parse_str(id).expect("Failed to create ObjectId");

        let query = doc! {
            "_id": oid,
        };

        match db
            .db
            .collection::<Document>(Self::Entity::collection().as_str())
            .find_one(query, None)
        {
            Ok(document) => document,
            Err(_) => None,
        }
    }
}

pub struct DoctorRepository {}
pub struct ServiceRepository {}

pub struct Doctor {

}

pub struct Service {

}

impl MongoEntity for Service {
    fn  collection() -> String  {
        String::from("services")
    }
}

impl MongoEntity for Doctor {
    fn collection() -> String {
        String::from("doctors")
    }
}

impl MongoRepository for DoctorRepository {
    type Entity = Doctor;
}

impl MongoRepository for ServiceRepository {
    type Entity = Service;
}

pub fn date_to_string(datetime: &DateTime<Utc>) -> String {
    let date = datetime.date();
    let time = datetime.time();

    format!("{}/{}/{} {}",
            date.year(), date.month(), date.day(),
            time.format("%H:%M").to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    fn connected() {
        use crate::MongoDB;
        let mongo = MongoDB::init();
        let customers = mongo.get_customers();
        for customer in customers.iter() {
            println!("{} ", customer.name);
        }
    }
}
