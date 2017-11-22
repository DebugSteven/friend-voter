extern crate scraper;
extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate regex;

use std::env;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::*;

use scraper::{Html, Selector};
use csv::ReaderBuilder;
use regex::Regex;

#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    pub firstName: String,
    //pub middleName: Option<String>,
    pub lastName: String,
    //age: String,
    //pub sex: String,
    //pub city: String,
    //pub state: String
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Create a path to the desired file
    let voter_path = Path::new(&args[1]);
    let friend_path = Path::new(&args[2]);
    let display_voter = voter_path.display();
    let display_friend = friend_path.display();

    let mut friend_file = match File::open(&friend_path) {
        Err(why) => panic!("couldn't open {}: {}", display_friend, why.description()),
        Ok(friend_file) => friend_file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut f = String::new();
    match friend_file.read_to_string(&mut f) {
        Err(why) => panic!("couldn't read {}: {}", display_friend, why.description()),
        Ok(_) => print!("Got friend contents from: {}\n", display_friend),
    };

    print!("We are going to compare the voter file {}", display_voter);
    println!(" to your friends list {}\n", display_friend);
    println!("Please hold on for results");

    let friends = parse_friends(f);
    parse_voters(&args[1], friends);

    println!("We're done! Check the folder this program ran in and you'll see the generated lists.")
}

fn parse_friends(friend_contents: String) -> Vec<String>{
    let doc = Html::parse_document(&friend_contents);
    let selector = Selector::parse("ul").unwrap();

    let ul = doc.select(&selector).skip(1).next().unwrap();
    let text = ul.text().collect::<Vec<_>>();

    //println!("Parsing ul: {:?}, {:?}", text, text.len());
    let re = Regex::new(r"[\[](?:.+)] [\(](?:.+)[\)]|[\(](?:.+)[\)]").unwrap();
    let ws = Regex::new(r"[ \t]+$").unwrap();

    //correctly parsed facebook names put into vector & reduced by regex
    let mut stack = Vec::new();

    for s in text.into_iter() {
        s.replace("\"","");
        let name = re.replace(s,"");
        let temp = &name.to_string();
        let namews = ws.replace(temp,"");
        //println!("{:?}", namews);
        stack.push(namews.to_string());
    }
    stack
}

fn parse_voters(v: &String, friends: Vec<String>){

    let voter_path = Path::new(v);
    let voter_file = File::open(&voter_path).unwrap();

    let mut yes = File::create("yeslist.txt").unwrap();
    let mut maybe = File::create("maybelist.txt").unwrap();

    let mut doc = csv::ReaderBuilder::new().from_reader(voter_file);
    for result in doc.deserialize() {
        let record: Record = result.unwrap();
        for guy in &friends {

            let mut iter = guy.split_whitespace();
            let first = iter.nth(0);
            let last = iter.last();
            
            if (last == Some(&record.lastName)) {
                if (first == Some(&record.firstName)) {
                    yes.write(guy.as_bytes()).unwrap();
                    yes.write(b"\n").unwrap();
                    yes.sync_all().unwrap();
                } else { 
                    maybe.write(guy.as_bytes()).unwrap();
                    maybe.write(b"\n").unwrap();
                    maybe.sync_all().unwrap();
                }
            } 
        }
    }
}
