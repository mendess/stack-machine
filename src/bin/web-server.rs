use actix_files::NamedFile;
use actix_web::{
    get,
    web::{self, Query},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use chrono::Utc;
use itertools::Itertools;
use stack_machine::run_with_input;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, Cursor},
    net::IpAddr,
};
use tokio::sync::mpsc::{self, channel};

#[derive(Debug)]
struct Request {
    ip: IpAddr,
    program: Program,
    result: String,
}

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
        .cloned()
        .unwrap_or_else(|| format!("{}", ip));
    let mut wrt = csv::WriterBuilder::new()
        .delimiter(b':')
        .from_writer(File::create(FILE)?);
    for u in map.into_iter().map(User::from) {
        wrt.serialize(u)?;
    }
    Ok(r)
}

#[derive(serde::Deserialize, Debug, Clone)]
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
async fn run(
    req: HttpRequest,
    tx: web::Data<mpsc::Sender<Request>>,
    mut s: Query<Program>,
) -> impl Responder {
    s.input.retain(|c| c != '\r');

    let result = std::panic::catch_unwind(|| run_with_input(&s.s, &mut Cursor::new(&s.input)));

    if let Some(ip) = req.peer_addr().map(|x| x.ip()) {
        if let Err(e) = tx
            .send(Request {
                ip,
                program: s.0.clone(),
                result: match &result {
                    Ok(Ok(v)) => format!("Ok: [{}]", v.iter().format(",")),
                    Ok(Err(e)) => format!("\x1b[31mErr:\x1b[0m {:?}", e),
                    Err(e) => format!("\x1b[1;31mpanicked at\x1b[0m '{:?}'", e),
                },
            })
            .await
        {
            eprintln!("failed to send ip to storing: {e:?}");
        }
    }

    match result {
        Ok(Ok(v)) => HttpResponse::Ok().body(iframe!("[{}]", v.into_iter().format(","))),
        Ok(Err(e)) => HttpResponse::BadRequest().body(iframe!("{:?}", e)),
        Err(e) => HttpResponse::InternalServerError().body(iframe!("panicked at '{:?}'", e)),
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
    let (tx, mut rx) = channel::<Request>(200);
    tokio::spawn(async move {
        eprintln!("storing task starting up");
        while let Some(Request {
            ip,
            program,
            result,
        }) = rx.recv().await
        {
            let u = match tokio::task::spawn_blocking(move || store(ip)).await {
                Ok(Ok(u)) => u,
                Ok(Err(e)) => {
                    eprintln!("Failed to store ip: {:?}", e);
                    ip.to_string()
                }
                Err(e) => {
                    eprintln!("Failed to join store task: {:?}", e);
                    ip.to_string()
                }
            };
            eprintln!("{}@[{:?}] {:?} => {result}", u, Utc::now(), program);
        }
        eprintln!("storing task shutting down");
    });
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tx.clone()))
            .service(run)
            .service(favicon)
            .service(home)
    })
    .bind("0.0.0.0:2021")?
    .run()
    .await
}
