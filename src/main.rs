use std::env;

fn main() {
    println!("Connecting to hello world server");

    let context = zmq::Context::new();
    let socket = context.socket(zmq::REQ).unwrap();

    let server_host = env::var("ZMQ_SERVER_HOST").unwrap();
    let addr = format!("tcp://{}:9702", server_host);

    let socks_proxy = env::var("ZMQ_SOCKS_PROXY");
    if socks_proxy.is_ok() {
        let proxy = socks_proxy.unwrap();
        println!("Use socks proxy: {}", &proxy);
        let result = socket.set_socks_proxy(Some(&proxy.as_str()));

        if result.is_err() {
            println!("socks error: {}", result.unwrap_err())
        }
    }

    assert!(socket.connect(&addr).is_ok());

    let mut msg = zmq::Message::new();

    for request_nbr in 0..9 {
        println!("Sending Hello with nr {}...", request_nbr);
        let hello_msg = format!("Hello nr {}", request_nbr);
        socket.send(&hello_msg, 0).unwrap();

        socket.recv(&mut msg, 0).unwrap();
        println!("Received World {}: {}", msg.as_str().unwrap(), request_nbr);
    }
}
