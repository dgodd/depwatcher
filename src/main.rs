#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate reqwest;

mod errors {
    use reqwest;
    error_chain!{
        foreign_links {
            Reqwest(reqwest::Error);
        }
    }
}
use errors::*;

#[derive(Serialize, Deserialize, Debug)]
struct External {
  number: String,
  sha: String,
  prerelease: bool,
}
#[derive(Serialize, Deserialize, Debug)]
struct Check {
  #[serde(rename = "ref")]
  version: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct In {
  #[serde(rename = "ref")]
  version: String,
  sha: String,
}

fn get(name: &str) -> Result<Vec<External>> {
    let mut resp = reqwest::get(&format!("https://rubygems.org/api/v1/versions/{}.json", name))?;
    if !resp.status().is_success() {
        return Err(format!("Received non success from rubygems.org: {}", resp.status()).into())
    }
    resp.json().map_err(|e| Error::from(e))
}

fn check(name: &str) -> Result<Vec<Check>> {
    let mut data: Vec<Check> = get(name)?.iter()
        .filter(|x| !x.prerelease)
        .take(10)
        .map(|x| Check { version: x.number.clone(), })
        .collect();
    data.reverse();
    Ok(data)
}

fn find(name: &str, number: &str) -> Result<In> {
    let data: Vec<External> = get(name)?;
    match data.iter().find(|x| x.number == number) {
        Some(x) => Ok(In { version: x.number.clone(), sha: x.sha.clone() }),
        None => Err("Could not find ref".into()),
    }
}

fn main() {
    match check("bundler") {
        Ok(d) => println!("{}", serde_json::to_string(&d).expect("Could not serialize")),
        Err(e) => {
            println!("Error: {}", e);
            ::std::process::exit(1);
        },
    }

    match find("bundler", "1.15.4") {
        Ok(d) => println!("{}", serde_json::to_string(&d).expect("Could not serialize")),
        Err(e) => {
            println!("Error: {}", e);
            ::std::process::exit(1);
        },
    }
}
