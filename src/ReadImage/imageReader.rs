use serde::{Deserialize, Serialize};
use curl::easy::Easy;
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Image {
    url: String,
}

pub fn getAnImage(uri: String) -> String{
    let mut handle = Easy::new();
    handle.url(&uri).unwrap();
    let mut html: String = String::new();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|data| {
            html = String::from_utf8(Vec::from(data)).unwrap();
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    
    let v: Image = serde_json::from_str(&html).unwrap();

    return v.url;
}