use actix_web::{App, HttpServer};

mod routes;
use routes::*;


#[actix_web::main]
async fn main()-> std::io::Result<()> {
    let server: actix_web::dev::Server = HttpServer::new(||{
        App::new()
        .service(home)
        .service(hello_user)
    }).bind(("127.0.0.1",8000))?.run();
    println!("Server running at 127.0.0.1:8000");
    server.await
}
