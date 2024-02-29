use actix_web::{test, App};
use events_finder::{create_app_config, create_app_data};

static TEST_CONFIG_FILE: &str = "config.yml";

#[actix_web::test]
async fn test_get_health() {
    let app_data = create_app_data(TEST_CONFIG_FILE);
    let app =
        test::init_service(App::new().configure(move |cfg| create_app_config(cfg, app_data))).await;

    let req = test::TestRequest::get().uri("/health").to_request();

    let resp = test::call_and_read_body(&app, req).await;

    assert_eq!(resp, "OK");
}
