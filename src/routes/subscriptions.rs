// use actix_web::web::Form;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;
use tracing::Instrument;
use crate::domain::{NewSubscriber, SubscriberEmail,  SubscriberName};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {  
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Self {email, name})
    }
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    )
)]
pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let new_subscriber = match form.0.try_into() {
        Ok(form) => form,
        Err(_) => return  HttpResponse::BadRequest().finish(),
    };

    match insert_subscriber(&pool, &new_subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO subscriptions (id, email, name, subscribed_at)
    VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        new_subscriber.email.as_ref(),
        new_subscriber.name.as_ref(),
        chrono::Utc::now(),
    )
    .execute(pool)
    .await
    .map(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

    
//     // let request_span = tracing::info_span!(
//     //     "Adding a new subscriber.",
//     //     %request_id,
//     //     subscriber_email = %form.email,
//     //     subscriber_name = %form.name
//     // );

//     // let _request_span_guard = request_span.enter();



//     // tracing::info!(
//     //     "request_id {} - Adding '{}' '{}' as a new subscriber.",
//     //     request_id,
//     //     form.email,
//     //     form.name
//     // );

//     // tracing::info!(
//     //     "request_id {} - Saving new subscriber details in the database",
//     //     request_id
//     // );

//     match sqlx::query!(
//         r#"
//         INSERT INTO subscriptions (id, email, name, subscribed_at)
//         VALUES ($1, $2, $3, $4)
//         "#,
//         Uuid::new_v4(),
//         form.email,
//         form.name,
//         chrono::Utc::now()
//     )
//     .execute(pool.as_ref())
//     .instrument(query_span)
//     .await
//     {
//         Ok(_) => {
//             tracing::info!(
//                 "request_id {} - New subscriber details have beeb saved",
//                 request_id
//             );
//             HttpResponse::Ok().finish()
//         },

//         Err(e) => {
//             tracing::error!(
//                 "Failed to execute query: {:?}",
//                 e
//             );
//             HttpResponse::InternalServerError().finish()
//         }
//     }
// }
