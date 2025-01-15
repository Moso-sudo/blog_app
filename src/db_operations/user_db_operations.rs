use diesel::PgConnection;
use crate::models::ui::RegisterForm;

pub fn ad_user(new_user:RegisterForm, connection: &mut PgConnection)-> Result<bool, String>{
    let res -diesel.insert_into(crate::schema::users::table).
    values(new_user).get_result::<user>()(connection)
}