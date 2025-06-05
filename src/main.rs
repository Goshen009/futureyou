use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tokio::spawn(async {
        email_scheduler().await;
    });

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .with(tracing_subscriber::fmt::layer()
            .with_target(false)
        )
        .init();

    let database = futureyou::Database::connect().await?;
    let app = futureyou::load_app(database);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("Server started on http://{}/", addr);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn email_scheduler() {
    use futures::TryStreamExt;

    let db = futureyou::Database::connect().await.unwrap();
    let resendkit = futureyou::ResendKit::initialize();
    
    loop {
        let letters = db.fetch_letters_to_be_scheduled()
            .await
            .inspect_err(|e| eprintln!("Error fetching letters from db: {:?}", e))
            .ok();

        let mut scheduled_letters = Vec::new();

        if let Some(mut cursor) = letters {
            while let Some(letter) = cursor.try_next().await.unwrap_or(None) {
                if let Ok(result) = resendkit.schedule_email(letter).await {
                    scheduled_letters.push(result);
                };
            }
        }

        if !scheduled_letters.is_empty() {
            db.bulk_update_scheduled_letters(&scheduled_letters).await
                .inspect_err(|e| eprintln!("Error updating letters in db: {:?}. All ids are: {:?}", e, scheduled_letters))
                .ok();
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60)).await;
    }
}