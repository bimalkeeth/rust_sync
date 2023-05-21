use std::io::{BufReader, Write};
use std::net::{Shutdown, TcpStream};
use resp::{Decoder, Value};
use crate::RUDIS_DB;

pub fn handle_client(stream: TcpStream) {
    let mut stream = BufReader::new(stream);
    let decoder = Decoder::new(&mut stream).decode();

    match decoder {
        Ok(v) => {
            let reply = process_client_request(v);
            stream.get_mut().write_all(&reply).unwrap();
        }
        Err(e) => {
            println!("invalid command :{:?}", e);
            let _ = stream.get_mut().shutdown(Shutdown::Both);
        }
    }
}

fn process_client_request(decoded_message: Value) -> Vec<u8> {
    let reply = if let Value::Array(v) = decoded_message {
        match &v[0] {
            Value::Bulk(ref s) if s == "GET" || s == "get" => handle_get(v),
            Value::Bulk(ref s) if s == "SET" || s == "set" => handle_set(v),
            other => unimplemented!("{:?} is not supported as of now", ),
        }
    } else {
        Err(Value::Error("invalid command".to_string()))
    };

    match reply {
        Ok(r) | Err(r) => r.encode(),
    }
}

fn handle_set(v: Vec<Value>) -> Result<Value, Value> {
    let v = v.iter().skip(1).collect::<Vec<_>>();
    if v.is_empty() || v.len() < 2 {
        return Err(Value::Error("expected two argument for SET command".to_string()));
    }

    match (&v[0],&v[1]) {
        (Value::Bulk(k),Value::Bulk(v))=>{
            let _ = RUDIS_DB.lock().unwrap().insert(k.to_string(),v.to_string());
        }
        _ => unimplemented!("set not implemented for {:?}",v),
    }

    Ok(Value::String("OK".to_string()))

}

fn handle_get(v: Vec<Value>) -> Result<Value, Value> {
    let v = v.iter().skip(1).collect::<Vec<_>>();

    if v.is_empty() {
        return Err(Value::Error("expected one argument for command".to_string()));
    }

    let db_ref = RUDIS_DB.lock().unwrap();
    let reply = if let Value::Bulk(ref s )=&v[0]{
        db_ref.get(s).map(|e|{
            return Value::Bulk(e.to_string())
        }).unwrap_or(Value::Null)
    }else{
        Value::Null
    };

    Ok(reply)
}
