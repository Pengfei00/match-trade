use actix_web::{App, HttpRequest, HttpServer, error, web,HttpResponse};
use match_trade::Engine;
mod websocket;
mod http;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut engine: Engine = Engine::new();
    {
        engine.add_book("BTC/DOGE", None);
    }
    let app_data = web::Data::new(engine);

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
        .app_data(web::JsonConfig::default().limit(4096).error_handler(json_error_handler))
            .app_data(app_data.clone())
            .route("/trade",web::to(http::trade))
            .route("/cancel",web::to(http::cancel))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    let detail = err.to_string();
    let response = match &err {
      error::JsonPayloadError::Deserialize(err1) => {
        HttpResponse::BadRequest().content_type("text/plain").body(format!("missing fields: {}", err1.to_string()))
      }
      _ => HttpResponse::BadRequest().content_type("text/plain").body(detail),
    };
  
    error::InternalError::from_response(err, response).into()
  }