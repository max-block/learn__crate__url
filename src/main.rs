use url::Url;
use std::error::Error;

fn main() ->Result<(), Box<dyn Error>>{
    let url = "mongodb://localhost/test?a=1";
    let parsed = Url::parse(url)?;

    println!("path: {}", parsed.path()); // /path

    Ok(())
}
