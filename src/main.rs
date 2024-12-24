use actix_web::{post, App, HttpServer, Responder};
use chrono::Utc;
use scylla;
use scylla::frame::value::CqlTimeuuid;
use scylla::transport::errors::QueryError;
use scylla::{QueryResult, Session, SessionBuilder};
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::Write;
use std::time::Duration;
use uuid::v1::{Context, Timestamp};
use uuid::Uuid;

// Missing the timeuuid because it gets added when we write to ScyllaDB
#[derive(Serialize, Deserialize, Debug)]
struct Reading {
    name: String,
    #[serde(rename = "auth-code")]
    authcode: String,
    temp: f32,
    humidity: f32,
    pressure: f32,
}

fn create_timeuuid() -> CqlTimeuuid {
    let ts = Timestamp::now(Context::new(0));
    CqlTimeuuid::from(Uuid::new_v1(ts, &[1,2,3,4,5,6]))
}

async fn insert_reading(session: &Session, datapoint: &Reading) -> Result<QueryResult, QueryError> {

    let time_stamp = Utc::now().timestamp() as i32;
    session
        .query("INSERT INTO raspi_sensing.temps (name,time,\"auth-code\",humidity,id,pressure,temp) VALUES (?,?,?,?,?,?,?)",
               (&datapoint.name, time_stamp, &datapoint.authcode, &datapoint.humidity, create_timeuuid(), &datapoint.pressure, &datapoint.temp,))
                .await

}

async fn create_scylla_conn(scylla_uri: &str) -> Session {
    SessionBuilder::new()
        .known_node(scylla_uri)
        //.known_nodes(uri) Can add more known Scylla cluster nodes
        .connection_timeout(Duration::from_secs(3))
        .cluster_metadata_refresh_interval(Duration::from_secs(10))
        .build()
        .await
        .unwrap()
}

#[derive(Serialize, Deserialize)]
struct Temp {
    temp: f32,
    time: i32
}

#[derive(Serialize, Deserialize)]
struct Hist {
    temps: Vec<f32>,
    times: Vec<i32>
}

#[post("/posttemp")]
async fn reading_post(body: String) -> impl Responder {

    let session = create_scylla_conn("scylla.lan").await;

    let _ = std::io::stdout().flush();

    let datapoint = match serde_json::from_str(&*body) {
        Ok(reading) => reading,
        Err(e) => {
            println!("Could not parse body to Reading");
            return format!("Error parsing body: {}", e.to_string());
        }
    };

    match insert_reading(&session, &datapoint).await {
        Ok(_) => (),
        Err(e) => {
            println!("Failed to insert reading: {}", e);
            return format!("Failed to insert reading: {}", e.to_string());
        }
    };

    let _ = std::io::stdout().flush();
    "Success".to_string() // Returns success if everything goes through

}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {

    let port = 0;
    let protocol = "http";
    let scylla_host = "";
    let bind_address = "";
    let keyspace_name = "";

    let session = create_scylla_conn(scylla_host).await;
    session.use_keyspace(keyspace_name, false)
        .await.
        expect("Failed to set keyspace");
    println!("Created ScyllaDB connection! ");

    println!("Listening for {} on port {}...",protocol,port);
    HttpServer::new(|| {
        App::new()
            //.app_data(web::Data::new(&session))
            .service(reading_post)
    })
    .bind((bind_address, port))?
    .run()
    .await
}