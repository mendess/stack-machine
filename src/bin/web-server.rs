use actix_files::NamedFile;
use actix_web::{get, web::Query, App, HttpRequest, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use itertools::Itertools;
use stack_machine::run_with_input;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, Cursor},
    net::{IpAddr, Ipv4Addr},
};

#[derive(serde::Serialize, serde::Deserialize)]
struct User {
    ip: IpAddr,
    user: Option<String>,
}

impl From<(IpAddr, Option<String>)> for User {
    fn from((ip, user): (IpAddr, Option<String>)) -> Self {
        Self { ip, user }
    }
}

fn store(ip: IpAddr) -> io::Result<String> {
    static FILE: &str = "iptable";
    let mut map = match File::open(FILE) {
        Ok(f) => {
            let mut rdr = csv::ReaderBuilder::new().delimiter(b':').from_reader(f);
            let mut map = HashMap::new();
            for r in rdr.deserialize() {
                let User { ip, user } = r?;
                map.insert(ip, user);
            }
            map
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => Default::default(),
        Err(e) => return Err(e),
    };
    let r = map
        .entry(ip)
        .or_default()
        .as_ref()
        .map(|x| x.clone())
        .unwrap_or_else(|| format!("{}", ip));
    let mut wrt = csv::WriterBuilder::new()
        .delimiter(b':')
        .from_writer(File::create(FILE)?);
    for u in map.into_iter().map(User::from) {
        wrt.serialize(u)?;
    }
    Ok(r)
}

#[derive(serde::Deserialize, Debug)]
struct Program {
    s: String,
    input: String,
}

macro_rules! iframe {
    ($($arg:tt)*) => {
        format!("<p style=\"color: rgba(198,199,196,255)\">{}</p>", ::std::format_args!($($arg)*))
    }
}

#[get("/run")]
async fn run(req: HttpRequest, mut s: Query<Program>) -> impl Responder {
    s.input.retain(|c| c != '\r');
    let ip_or_def = |req: &HttpRequest| {
        format!(
            "{}",
            req.peer_addr()
                .map(|x| x.ip())
                .unwrap_or_else(|| Ipv4Addr::UNSPECIFIED.into())
        )
    };

    let u = match req.peer_addr().map(|x| x.ip()).map(store) {
        Some(Ok(u)) => u,
        Some(Err(e)) => {
            eprintln!("Failed to store ip: {:?}", e);
            ip_or_def(&req)
        }
        None => ip_or_def(&req),
    };
    eprintln!("{}@[{:?}] {:?}", u, Utc::now(), s);

    match run_with_input(&s.s, Cursor::new(&s.input)) {
        Ok(v) => HttpResponse::Ok().body(iframe!("[{}]", v.into_iter().format(","))),
        Err(e) => HttpResponse::BadRequest().body(iframe!("{:?}", e)),
    }
}

#[get("/favicon.ico")]
async fn favicon() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("static/favicon.ico")?)
}

#[get("/")]
async fn home() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().service(run).service(favicon).service(home))
        .bind("0.0.0.0:2021")?
        .run()
        .await
}
