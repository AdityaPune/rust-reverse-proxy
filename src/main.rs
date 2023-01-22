#![deny(warnings)]

use hyper::{server::conn::Http, service::service_fn};
use std::{net::SocketAddr};
use tokio::net::{TcpListener, TcpStream};
// use std::collections::HashMap;
// use std::time::{ Duration, Instant};
// use hyper::{body::HttpBody};
// use bytes::Bytes;
// use std::sync::{Arc, Mutex};


//A custom cache store
// struct Cache {
//     data: HashMap<String, (Instant, String)>,
//     ttl: Duration,
// }

// impl Cache {
//     fn new(ttl: Duration) -> Self {
//         Cache {
//             data: HashMap::new(),
//             ttl,
//         }
//     }

//     fn get(&self, key: &str) -> Option<&String> {
//         let now = Instant::now();
//         self.data.get(key).filter(|(timestamp, _)| now.duration_since(*timestamp) < self.ttl).map(|(_, v)| v)
//     }

//     fn set(&mut self, key: String, value: String) {
//         self.data.insert(key.to_string(), (Instant::now(), value.to_string()));
//     }
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let server_address = "blockstream.info/api/blocks";
    let in_addr: SocketAddr = ([127, 0, 0, 1], 3001).into();
    // let cache = Cache::new(Duration::from_secs(30));
    // let cache = Arc::new(Mutex::new(cache));

    let listener = TcpListener::bind(in_addr).await?;

    println!("Listening on http://{}", in_addr);
    println!("Proxying on http://{}", server_address);

    let server_clone = server_address.clone();
    loop {
        // let cache = Arc::clone(&cache);
        let (stream, _) = listener.accept().await?;

        //Do cache get here
        let service = service_fn(move |mut req| {
            // let cache = Arc::clone(&cache);
            let uri_string = format!(
                "http://{}{}",
                server_clone,
                req.uri()
                    .path_and_query()
                    .map(|x| x.as_str())
                    .unwrap_or("")
            );
            //Wanted to implement the fetching from cache mechanism in here but couldn't get to it as setting the value in cache wasn't working
            // match cache.get(&uri_string) {
            //     Some(value) => {
            //         // println!("From cache {value}");
            //         let response = Response::new();

            //         assert_eq!(response.status(), StatusCode::OK);
            //         assert_eq!(*response.body(), value.to_string());
            //         Ok(response)
            //     },
            //     None => ()
            // }
            let uri = uri_string.parse().unwrap();
            *req.uri_mut() = uri;

            let host = req.uri().host().expect("uri has no host");
            let port = req.uri().port_u16().unwrap_or(80);
            let addr = format!("{}:{}", host, port);
            // println!("{:?}",uri_string);

            async move {
                // let cache = Arc::clone(&cache);
                let client_stream = TcpStream::connect(addr).await.unwrap();

                let (mut sender, conn) =
                    hyper::client::conn::handshake(client_stream).await?;
                tokio::task::spawn(async move {
                    if let Err(err) = conn.await {
                        println!("Connection failed: {:?}", err);
                    }
                });

                //Attempted using the reqwest crate
                // let resp = match reqwest::blocking::get(&uri_string) {
                //     Ok(resp) => resp.text().unwrap(),
                //     Err(err) => panic!("Error: {}", err)
                // };

                let response = sender.send_request(req).await;                
                // const MAX_ALLOWED_RESPONSE_SIZE: u64 = 1024;
                // let response_content_length = match response.unwrap().body().size_hint().upper() {
                //     Some(v) => v,
                //     None => MAX_ALLOWED_RESPONSE_SIZE + 1 // Just to protect ourselves from a malicious response
                // };
                
                // let mut body_bytes = Bytes::from(&b"Hello world"[..]);
                // if response_content_length < MAX_ALLOWED_RESPONSE_SIZE {
                //     body_bytes = hyper::body::to_bytes(response.unwrap().into_body()).await?;
                //     println!("body: {:?}", body_bytes);
                // }
                // let body_string = String::from_utf8(body_bytes.to_vec()).unwrap();
                // cache.lock().unwrap().set(uri_string, body_string);
                response
            }
        });

        tokio::task::spawn(async move {
            if let Err(err) = Http::new()
                .serve_connection(stream, service)
                .await
            {
                println!("Failed to servce connection: {:?}", err);
            }
        });
    }
    
}