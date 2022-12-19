use rustworld_2d;
use tokio::signal;
use futures_util::future::{self, Either, FutureExt};


#[tokio::main]
async fn main() {
    // say hello
    println!("\nHello, rust world!\n");

    // configer web server
    let web_server_adress = "127.0.0.1:5000";
    let server_future = gotham::init_server(web_server_adress, ||Ok(rustworld_2d::say_hello));

    // configer Ctrl-C future
    let ctrl_c_future = async {
        signal::ctrl_c().await.expect("failed to listen for Ctrl-C");
        println!("Ctrl-C pressed.");
    };

    // start web server and Ctrl-C handler
    println!("Start Gotham Web Server. Listening at {}", web_server_adress);
    let exit_reason = future::select(server_future.boxed(), ctrl_c_future.boxed()).await;

    // exit program
    if let Either::Left((Err(err), _ )) = exit_reason {
        println!("Error starting web server: {}", err);
    } else if let Either::Right(_) = exit_reason {
        println!("web server closed");
    } else {
        println!("unknown exit reson! What the heck?");
    }

}


