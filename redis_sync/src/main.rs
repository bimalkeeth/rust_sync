mod handle_request;

use std::collections::HashMap;
use std::env;
use std::net::TcpListener;
use std::sync::Mutex;
use lazy_static::lazy_static;



type STORE = Mutex<HashMap<String,String>>;

lazy_static!{
    static ref RUDIS_DB: STORE = Mutex::new(HashMap::new());
}


fn main() {
    let addr = env::args()
        .skip(1)
        .next().unwrap_or("127.0.0.1:6378".to_owned());

    let listener= TcpListener::bind(&addr).unwrap();

    println!("rudis_sync listening on {} ...",addr);

    for stream in listener.incoming(){
        let stream =stream.unwrap();

        println!("new connection from {:?}",stream);

        handle_request::handler::handle_client(stream);
    }
}
