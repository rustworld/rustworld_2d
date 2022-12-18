use rustworld_2d;
use gotham::test::TestServer;
use gotham::hyper::StatusCode;


#[test]
fn hello_world_web_test() {
    let test_server = TestServer::new(||Ok(rustworld_2d::say_hello)).unwrap();
    let response    = test_server.client().get("http://localhost").perform().unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.read_body().unwrap();
    assert_eq!(body, b"Hello Gotham");

}