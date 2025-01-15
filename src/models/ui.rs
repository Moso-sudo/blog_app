use askama::Template;
use diesel::prelude::*

#[derive(Template)]
#[template(path="login.html")]
pub struct LoginTemplate{
    pub error:Option<String>
}

#[derive(Template)]
#[template(path="register.html")]
pub struct RegisterTemplate{
    pub error:Option<String>
}
#[derive(Template)]
#[template(path="dashboard.html")]
pub struct DashboardTemplate{
    pub error:Option<String>
}
#[derive(Deserialize, Serialize)]
pub struct LoginForm{
  pub  email: String
  pub password:String
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name  crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct RegisterForm{
    pub name : String,
    pub  email: String
    pub password:String
  }