use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use serde::{Serialize, Deserialize};
use serde_json;

extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

#[derive(Serialize, Deserialize, Debug)]
struct Collaborator {
    slug: String,
    name: String,
    email: String,
    last_used: bool,
}

impl Collaborator {
    fn new(slug: &str, name: &str, email: &str) -> Collaborator {
        Collaborator {
            slug: slug.to_string(),
            name: name.to_string(),
            email: email.to_string(),
            last_used: false,
        }
    }
}

fn main() -> std::io::Result<()> {
    let add_command = String::from("add");
    let remove_command = String::from("remove");
    let list_command = String::from("list");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2  {
        println!("do something different");

    } else {
        let slug_or_command = &args[1];
        let mut existing_collabs = collaborators_hash();

        if slug_or_command == &add_command {
            add_collaborator();

        } else if slug_or_command == &remove_command {
            remove_collaborator();

        } else if slug_or_command == &list_command {
            for collab in existing_collabs {
                println!("{}", collab.slug);
            }

        } else {
            find_and_copy_collaborator()
        }

    }

    Ok({})
}

fn add_collaborator() {
    let args: Vec<String> = env::args().collect();
    let mut existing_collabs = collaborators_hash();

    if args.len() < 5 {
        println!("\"add\" help: provide `slug`, `name` and `email`")
    } else {
        let collab = Collaborator::new(&args[2], &args[3], &args[4]);
        existing_collabs.push(collab);
        let to_write = serde_json::to_string(&existing_collabs).expect("something went wrong");

        fs::write("collaborators.json", &to_write);
    }
}

fn remove_collaborator() {
    let args: Vec<String> = env::args().collect();
    let mut existing_collabs = collaborators_hash();

    if args.len() < 3 {
        println!("\"remove\" help: provide `slug`")
    } else {
        let slug = &args[2];
        let no_match = format!("{slug} collaborator not found").to_string();
        let matches = collaborators_by_slug(&slug);
        if matches.len() > 0 {
            let index = existing_collabs.iter().position(|r| r.slug == matches[0].slug).unwrap();
            existing_collabs.remove(index);
            let to_write = serde_json::to_string(&existing_collabs).expect("something went wrong");

            fs::write("collaborators.json", &to_write);
            println!("{} removed!", &slug);
        } else {
            println!("{}", no_match);
        }
    }
}

fn find_and_copy_collaborator() {
    let args: Vec<String> = env::args().collect();

    let slug = &args[1];
    let no_match = format!("{slug} collaborator not found").to_string();
    let matches = collaborators_by_slug(&slug);

    if matches.len() > 0 {
        let collab = &matches[0];
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let formatted = format!("{} <{}>", collab.name, collab.email);
        ctx.set_contents(formatted.to_owned()).unwrap();
        println!("{} copied!", formatted);
    } else {
        println!("{}", no_match);
    }
}
fn collaborators_by_slug(slug: &str) -> Vec<Collaborator> {
    let collabs = collaborators_hash();
    collabs
        .into_iter()
        .filter(|c| &c.slug == slug)
        .collect()
}

fn collaborators_hash() -> Vec<Collaborator> {
    let file = File::open("collaborators.json").expect("can't find collaborators.json file");
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).expect("JSON was not well-formatted")
}