use domain::QuestionId;
use std::str::FromStr;
use use_cases::question;
use warp::{
    filters::cors::CorsForbidden,
    http::{Method, StatusCode},
    reject::{Reject, Rejection},
    Filter,
};

#[derive(Debug)]
struct InvalidId;

impl Reject for InvalidId {}

//

async fn return_error(r: Rejection) -> Result<impl warp::Reply, Rejection> {
    dbg!(&r);
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(_invalid_id) = r.find::<InvalidId>() {
        Ok(warp::reply::with_status(
            "Not valid ID".to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

async fn get_question() -> Result<impl warp::Reply, warp::Rejection> {
    let id = QuestionId::from_str("1").expect("Invalid ID");

    let question = question::new(
        id,
        "First question".to_owned(),
        "Content of question".to_owned(),
        Some(vec!["faq".to_owned()]),
    );

    match question.id.0.parse::<i32>() {
        Err(_) => Err(warp::reject::custom(InvalidId)),
        Ok(_) => Ok(warp::reply::json(&question)),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = 3000;

    //

    let cors = warp::cors()
        .allow_any_origin()
        // .allow_origin("https://google.com")
        .allow_header("content-type")
        .allow_methods(&[
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT,
            Method::OPTIONS,
        ]);

    // Handle OPTIONS requests
    let options_handler = warp::options()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .map(|| warp::reply::with_status("", StatusCode::NO_CONTENT));

    // Handle GET requests
    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_question)
        .recover(return_error);

    // Combine routes
    let routes = options_handler.or(get_items).with(cors);

    warp::serve(routes).run(([127, 0, 0, 1], port)).await;

    Ok(())
}
