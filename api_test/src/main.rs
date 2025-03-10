use axum::{Router, routing::get, extract::{Query, State}};
use serde::Deserialize;
use tower_http::cors::{CorsLayer, Any};

#[derive(Debug, Deserialize)]
pub struct Params {
    intensity: Option<u32>,
}

use rppal::pwm::{Channel, Polarity, Pwm};

use std::sync::{Arc, Mutex};

mod pwm;

#[derive(Clone, Debug)]
pub struct SharedState {
    pwm: Arc<Pwm>,
}

#[tokio::main]
async fn main() {
    
    println!("Starting API server");

    let shared_state = SharedState {
        pwm: Arc::new(Pwm::with_frequency(Channel::Pwm0, 150.0, 0.99, Polarity::Normal, true).expect("Could not create PWM object"))
    };

    let app = Router::new()
        .route("/bathroom_led", get(bathroom_led))
        .layer(CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
        )
        .with_state(Arc::new(shared_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5123").await.unwrap();
    
    println!("Server is running on 0.0.0.0:5000");

    axum::serve(listener,app).await.unwrap();
    


}

async fn bathroom_led(Query(params): Query<Params>, State(state): State<Arc<SharedState>>) -> String {
    
    println!("intensity: {:?}", params.intensity);
    println!("state: {:?}", state);
    let _ = pwm::set_pwm(state.clone().pwm.clone(), params.intensity);

    "Calling the led".to_string()
}