use canapi::{Error, Provider};
use chrono::NaiveDateTime;
use diesel::{self, RunQueryDsl, QueryDsl, ExpressionMethods};
use openssl::rand::rand_bytes;

use plume_api::apps::AppEndpoint;
use Connection;
use schema::apps;

#[derive(Clone, Queryable)]
pub struct App {
    pub id: i32,
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: Option<String>,
    pub website: Option<String>,    
    pub creation_date: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name= "apps"]
pub struct NewApp {
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: Option<String>,
    pub website: Option<String>,    
}

impl Provider<Connection> for App {
    type Data = AppEndpoint;

    fn get(conn: &Connection, id: i32) -> Result<AppEndpoint, Error> {
        unimplemented!()
    }

    fn list(conn: &Connection, query: AppEndpoint) -> Vec<AppEndpoint> {
        unimplemented!()
    }

    fn create(conn: &Connection, data: AppEndpoint) -> Result<AppEndpoint, Error> {
        let mut id = [0; 32];
        rand_bytes(&mut id).expect("Error while generating client id");
        let client_id = id.into_iter().fold(String::new(), |res, byte| format!("{}{:x}", res, byte)); 
        
        let mut secret = [0; 32];
        rand_bytes(&mut secret).expect("Error while generating client secret");
        let client_secret = secret.into_iter().fold(String::new(), |res, byte| format!("{}{:x}", res, byte)); 
        let app = App::insert(conn, NewApp {
            name: data.name.expect("App::create: name is required"),
            client_id: client_id,
            client_secret: client_secret,
            redirect_uri: data.redirect_uri,
            website: data.website,
        });

        Ok(AppEndpoint {
            id: Some(app.id),
            name: Some(app.name),
            client_id: Some(app.client_id),
            client_secret: Some(app.client_secret),
            redirect_uri: app.redirect_uri,
            website: app.website,
        })
    }

    fn update(conn: &Connection, id: i32, new_data: AppEndpoint) -> Result<AppEndpoint, Error> {
        unimplemented!()
    }
    
    fn delete(conn: &Connection, id: i32) {
        unimplemented!()
    }
}

impl App {
    get!(apps);
    insert!(apps, NewApp);
} 
