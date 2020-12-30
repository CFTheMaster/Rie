use serde::{Deserialize, Serialize};
use curl::easy::Easy;

#[derive(Serialize, Deserialize)]
struct Image {
    url: String,
}

pub fn getAnImage(uri: String) -> String{
    let mut url = "https://api.computerfreaker.cf/v1/";
    url.append(uri);

    let mut list = curl::easy::List::new();
    list.append("User-Agent:Rie#5977/0.1.0 (Serenity-Rust) CFsAPI/706219430912327742")
        .expect("Couldn't set User-Agent");
    let mut handle = Easy::new();
    handle.url(&url).unwrap();
    handle.http_headers(list)
        .expect("couldn't set headers!!!");
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