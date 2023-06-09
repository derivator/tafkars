use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

use std::env;

pub mod api_translation;
pub mod endpoints;
pub mod web_config;
pub use api_translation::*;
pub use endpoints::*;
pub use web_config::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init(); // test with RUST_LOG=info to see requests

    let lemmy_url: String = env::args()
        .nth(1)
        .expect("please providy a lemmy instance url as a cmd arg");
    let config = GatewayConfig { lemmy_url };

    let app_state = AppState {
        http_client: Default::default(),
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(web_root)
            .service(frontpage)
            .service(subreddit)
            .service(comments_for_post)
            .app_data(app_state.clone())
            .app_data(config.clone())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
