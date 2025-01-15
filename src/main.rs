
mod models;
mod controllers::auth{login_user,login_page,register_page, register_user};
mod db_operation;
use std::sync::Mutex;
use actix_web{App, web, HttpServer, Responder, httpResponse};
use dotenvy::dotenv;
use models::app_state;
use crate::models::app_state::AppState;
use db_operation::db::establish_connection;

#[actix_web::main]
async fn main()=> std::io::Result<()> {
    dotenv().ok();
    
    let host = env::var("HOST").expect("HOST not found ");
    let port = env::var("PORT").expect("PORT not found ");
    let port_host =fomart!("{},{}", host, port)
httpServer::new(move ||{
   let app_state= web::Data::new(AppState{
    db_connection:Mutex::new(establish_connection()){

    }
   });
    App::new().app_data(app_state.clone)
    .route("/login", web::get().to(login_page))
    .route("/login", web::post().to(login_user))
    .route("/Register", web::get().to(register_page))
    .route("/Register", web::get().to(register_user))
}).bind(port_host)?
.run()
.await
}
