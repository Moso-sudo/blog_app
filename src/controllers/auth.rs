use crate::models::ui::{LoginTemplate, RegisterTemplate,DashboardTemplate}
use models::ui:{LoginForm, RegisterForm}
use actix_web{Responder, web, HttpServer}
use askama::Template
use actix_web::{web, HttpResponse, Responder}
use bycript::{hash, DEFAULT_COST}
pub async fn login_page(error:Option<String>) ->impl Responder{
    let template LoginTemplate{error};
    httpResponse::ok().content_type("text/html").body(template.render().unwrap())

}
pub async fn register_page(error:Option<String>) ->impl Responder{
    let template RegisteTemplate{error};
    httpResponse::ok().content_type("text/html").body(template.render().unwrap())
    
}

pub async fn login_user(fom: web::Form<LOginForm>, state: web:Data<Appstate>)->impl Responder{
    let template LoginTemplate{error: some("To be implemented".to_string())};
    httpResponse::ok().content_type("text/html").body(template.render().unwrap())

}
pub async fn register_user(fom: web::Form<RegisterForm>, state: web:Data<Appstate>)->impl Responder{
    //validate payload
    if form.email.is_empty()||form.password.is_empty(){
        let template RegisterTemplate{error: some("Ensure  all fields are filled".to_string())};
        httpResponse::ok().content_type("text/html").body(template.render().unwrap())

    }
    let hash_password = match hash(&form.password, DEFAULT_COST){
        o(hash)=>hash,
        Err(_){
            let template RegisterTemplate{error: some("an error occurerd".to_string())};
        httpResponse::ok().content_type("text/html").body(template.render().unwrap())

        }

    };
    

}