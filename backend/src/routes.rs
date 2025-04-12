use crate::AppState;
use crate::models;
use actix_web::{Either, Error, HttpRequest, HttpResponse, get, post, rt, web};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;
use mongodb::bson::DateTime;
use mongodb::{
    Collection, bson,
    bson::{Document, doc},
};
use random_word::Lang;

#[post("/createrun")]
pub async fn create_run(data: web::Data<AppState>) -> Either<String, HttpResponse> {
    let db = data.mongodb_client.database(&data.db_name);
    let runs: Collection<models::RunData> = db.collection("runs");
    let mut potential_id: String;

    // loop until unused id
    loop {
        let word1 = random_word::get(Lang::En);
        let word2 = random_word::get(Lang::En);
        let num = rand::random_range(0..=99);
        potential_id = format!("{0}-{1}-{2}", word1, word2, num);

        let db_res = runs
            .find_one(doc! {"id_string": &potential_id})
            .await
            .expect("Create run failed to query database");

        if db_res.is_none() {
            break;
        }
    }

    let new_run = models::RunData {
        id_string: potential_id.clone(),
        players: Vec::new(),
        encounters: Vec::new(),
        created_at: DateTime::now(),
        updated_at: DateTime::now(),
    };
    let res = runs.insert_one(new_run).await;

    let id_string_or_error: Result<String, HttpResponse> = res
        .map_err(|e| {
            HttpResponse::InternalServerError().body(format!("Failed to insert run: {}", e))
        })
        .map(|_| potential_id);

    match id_string_or_error {
        Ok(s) => Either::Left(s),
        Err(e) => Either::Right(e),
    }
}

#[get("/run/{run_id}")]
pub async fn get_run(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Either<models::RunData, HttpResponse> {
    let db = data.mongodb_client.database(&data.db_name);
    let runs: Collection<Document> = db.collection("runs");
    let run_id: String = path.to_string();

    let db_res = runs.find_one(doc! {"id_string": &run_id}).await;

    let res: Result<models::RunData, HttpResponse> = db_res
        .map_err(|_| HttpResponse::InternalServerError().body("Database get run failed"))
        .and_then(|option_doc| {
            option_doc.ok_or_else(|| {
                HttpResponse::NotFound()
                    .body(format!("Could not find run with id string: {}", run_id))
            })
        })
        .and_then(|doc| {
            bson::from_document(doc).map_err(|e| {
                HttpResponse::InternalServerError()
                    .body(format!("Failed to deserialize run: {}", e))
            })
        });

    match res {
        Ok(data) => Either::Left(data),
        Err(e) => Either::Right(e),
    }
}

#[get("/update/{run_id}")]
pub async fn update_run(
    data: web::Data<AppState>,
    path: web::Path<String>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;
    let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20));

    rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    println!("{text}");
                }
                Ok(_) => {
                    println!("Received non-text OK");
                }
                Err(e) => {
                    println!("WS Error: {}", e);
                }
            }
        }
    });

    Ok(res)
}

#[get("/pingdb")]
pub async fn ping_db(data: web::Data<AppState>) -> HttpResponse {
    let res = data.mongodb_client.list_databases().await;

    match res {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::ServiceUnavailable().finish(),
    }
}
