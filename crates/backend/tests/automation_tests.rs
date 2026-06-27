use actix_web::{test, web, App};
use actix_http::Request;
use backend::routes;
use backend::state::AppState;
use shared::models::{Automation, CreateAutomationRequest};
use mongodb::Client;
use rumqttc::{AsyncClient, MqttOptions};

async fn setup_test_app() -> impl actix_web::dev::Service<
    Request,
    Response = actix_web::dev::ServiceResponse,
    Error = actix_web::Error,
> {

    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    let db = client.database("test_db");

    // Clear DB
    db.collection::<Automation>("automations")
        .drop()
        .await
        .unwrap();

    // Create test MQTT client
    let mqttoptions = MqttOptions::new("test_client", "localhost", 1883);
    let (mqtt_client, _eventloop) = AsyncClient::new(mqttoptions, 10);
    let mqtt = backend::mqtt::MqttClient { client: mqtt_client };

    let state = AppState {
        db: db.clone(),
        mqtt,
        config: backend::config::AppConfig::from_env().unwrap(),
    };

    test::init_service(
        App::new()
            .app_data(web::Data::new(state))
            .configure(routes::init),
    )
    .await
}

#[actix_web::test]
async fn test_automation_flow() {
    let app = setup_test_app().await;

    // 1. Create
    let req = test::TestRequest::post()
        .uri("/api/automations")
        .insert_header(("Authorization", "Bearer test_token"))
        .set_json(CreateAutomationRequest {
            home_id: "home1".to_string(),
            name: "Test Automation".to_string(),
            triggers: vec![],
            actions: vec![],
        })
        .to_request();

    let resp: actix_web::dev::ServiceResponse = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}