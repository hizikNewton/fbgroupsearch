use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    password: String,
}

pub async fn login(form: web::Form<FormData>,pool: web::Data<PgPool>,) -> HttpResponse {
    match sqlx::query!(
        r#"
        INSERT INTO fbsearch (id, email, name, searched_at)
        VALUES ($1, $2, $3, $4)
        "#,Uuid::new_v4(),form.email,form.password,Utc::now())
        // We use `get_ref` to get an immutable reference to the `PgConnection`
        // wrapped by `web::Data`.
        .execute(pool.get_ref())
        .await{
            Ok(_) => HttpResponse::Ok().finish(),
            Err(e) => {
                println!("Failed to execute query: {}", e);
                HttpResponse::InternalServerError().finish()
            }
        }
}
