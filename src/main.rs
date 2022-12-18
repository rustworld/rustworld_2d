use rustworld_2d;


fn main() {
    let web_server_adress = "127.0.0.1:5000";

    println!("\nHello, rust world!\n");
    println!("Start Gotham Web Server. Listening at {}", web_server_adress);
    gotham::start(web_server_adress, ||Ok(rustworld_2d::say_hello)).expect("failed to start gotham web server");
}


