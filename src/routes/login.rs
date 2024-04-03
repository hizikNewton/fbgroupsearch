use actix_web::{web, HttpResponse};
use sqlx::PgConnection;
use uuid::Uuid;
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    password: String,
}

pub async fn login(form: web::Form<FormData>,connection: web::Data<PgConnection>,) -> HttpResponse {
    sqlx::query!(
        r#"
        INSERT INTO fbsearch (id, email, name, searched_at)
        VALUES ($1, $2, $3, $4)
        "#,Uuid::new_v4(),form.email,form.password,Utc::now())
        // We use `get_ref` to get an immutable reference to the `PgConnection`
        // wrapped by `web::Data`.
        .execute(connection.get_ref())
        .await;
    HttpResponse::Ok().finish()
}
