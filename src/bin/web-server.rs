use actix_files::NamedFile;
use actix_web::{get, web::Query, App, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use itertools::Itertools;
use stack_machine::run_with_input;
use std::io::{self, Cursor};

#[derive(serde::Deserialize, Debug)]
struct Program {
    s: String,
    input: String,
}

#[get("/run")]
async fn run(req: HttpRequest, s: Query<Program>) -> impl Responder {
    eprintln!(
        "{}@[{:?}] {:?}",
        req.peer_addr()
            .map(|x| x.ip())
            .unwrap_or_else(|| std::net::Ipv4Addr::UNSPECIFIED.into()),
        Utc::now(),
        s
    );
    match run_with_input(&s.s, Cursor::new(&s.input)) {
        Ok(v) => HttpResponse::Ok().body(format!("[{}]", v.into_iter().format(","))),
        Err(e) => HttpResponse::BadRequest().body(format!("{:?}", e)),
    }
}

#[get("/")]
async fn home() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(run).service(home))
        .bind("0.0.0.0:2021")?
        .run()
        .await
}
