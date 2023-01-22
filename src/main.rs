use reqwest;
use std::collections::HashMap;
use std::time::{ Duration, Instant};


struct Cache {
    data: HashMap<String, (Instant, String)>,
    ttl: Duration,
}

impl Cache {
    fn new(ttl: Duration) -> Self {
        Cache {
            data: HashMap::new(),
            ttl,
        }
    }

    fn get(&self, key: &str) -> Option<&String> {
        let now = Instant::now();
        self.data.get(key).filter(|(timestamp, _)| now.duration_since(*timestamp) < self.ttl).map(|(_, v)| v)
    }

    fn set(&mut self, key: &String, value: &String) {
        self.data.insert(key.to_string(), (Instant::now(), value.to_string()));
    }
}
fn main() {
    let mut cache = Cache::new(Duration::from_secs(30));
    loop{       
        println!("Enter a URL");
        let mut url = String::new();
        std::io::stdin().read_line(&mut url).expect("Expected a url");
        match cache.get(&url) {
            Some(value) => {
                println!("From cache {value}");
                continue
            },
            None => ()
        }
        let resp = match reqwest::blocking::get(&url) {
            Ok(resp) => resp.text().unwrap(),
            Err(err) => panic!("Error: {}", err)
        };
        cache.set(&url, &resp);
        println!("{}", resp);
    }
}