use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
// use std::io::Write;
use serde_json;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let add_command = String::from("add");
    let remove_command = String::from("remove");

    let args: Vec<String> = env::args().collect();
    let slug_or_command = &args[1];
    let mut hash = collaborators_hash();

    if slug_or_command == &add_command {
        if args.len() < 5 {
            println!("\"add\" help: provide `slug`, `name` and `email`")
        } else {
            let slug = &args[2];
            let name = &args[3];
            let email = &args[4];
            hash.insert(slug.to_string(), format!("{name} <{email}>"));

            let to_write = serde_json::to_string(&hash).expect("something went wrong");
            fs::write("collaborators.json", &to_write);
        }

    } else if slug_or_command == &remove_command {
        if args.len() < 3 {
            println!("\"remove\" help: provide `slug`")
        } else {
            let slug = &args[2];
            hash.remove(slug);

            let to_write = serde_json::to_string(&hash).expect("something went wrong");
            fs::write("collaborators.json", &to_write);
        }

    } else {
        let slug = slug_or_command;

        let hash = collaborators_hash();
        let no_match = format!("{slug} collaborator not found").to_string();
        let matched = match hash.get(slug) {
            Some(val) => val,
            None => &no_match
        };
        println!("{}", matched);
    }

    Ok({})
}

fn collaborators_hash() -> HashMap<String, String> {
    let file = File::open("collaborators.json").expect("can't find collaborators.json file");
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).expect("JSON was not well-formatted")
}
