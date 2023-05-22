use actix_web::{web, HttpResponse};
use sqlx::PgPool;

pub async fn get_contacts(pool: web::Data<PgPool>) -> HttpResponse {
    match query_contacts(&pool).await {
        Ok(contacts) => HttpResponse::Ok().json(contacts),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[tracing::instrument(
    name = "Querying contacts from DB",
    skip(pool)
)]
async fn query_contacts(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        SELECT contact_id, display_name, address, city, state, zip_code, capacity, latitude, longitude, email, contact_form, age_range
        FROM contacts
        "#
    ).fetch_all(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execture query: {:?}", e);
        e
    })?;

    Ok(())
}