use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::{Serialize, Deserialize};
use home;
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
        let last_used = last_used_collaborator();
        if last_used.is_some() {
            copy_collaborator(&last_used.unwrap());
        } else {
            list_collaborators();
        }

    } else {
        let slug_or_command = &args[1];

        if slug_or_command == &add_command {
            add_collaborator();

        } else if slug_or_command == &remove_command {
            remove_collaborator();

        } else if slug_or_command == &list_command {
            list_collaborators();

        } else {
            find_and_copy_collaborator()
        }

    }

    Ok({})
}

fn config_file() -> String {
    let home_dir = home::home_dir().unwrap();
    let gcollab_dir = format!("{}/.gcollab", &home_dir.display());
    if !Path::new(&gcollab_dir).is_dir() {
        fs::create_dir(&gcollab_dir);
    }
    let file_path = format!("{}/collaborators.json", &gcollab_dir);

    if !std::path::Path::new(&file_path).exists() {
        let empty: Vec<Collaborator> = Vec::new();
        let to_write = serde_json::to_string(&empty).expect("something went wrong");
        fs::write(&file_path, &to_write);
    }
    return file_path;
}

fn list_collaborators() {
    let existing_collabs = collaborators_hash();
    if existing_collabs.len() > 0 {
        for collab in existing_collabs {
            println!("{}", collab.slug);
        }
    } else {
        println!("Goose Egg");
        println!("Try 'gcollab add' to create some.");
    }
}

fn add_collaborator() {
    let args: Vec<String> = env::args().collect();
    let mut existing_collabs = collaborators_hash();
    let collab: Collaborator;

    if args.len() < 5 {
        collab = assited_creation();
    } else {
        collab = Collaborator::new(&args[2], &args[3], &args[4]);
    }

    let slug = collab.slug.clone();
    existing_collabs.push(collab);
    let to_write = serde_json::to_string(&existing_collabs).expect("something went wrong");

    let result = fs::write(config_file(), &to_write);
    if result.is_ok() {
        let saved = collaborators_by_slug(&slug);
        println!("{} added!", &saved[0].name);
        copy_collaborator(&saved[0]);
    } else {
        println!("epic fail")
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

            let _ = fs::write(config_file(), &to_write);
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
        copy_collaborator(&matches[0]);
        update_last_used(&matches[0]);
    } else {
        println!("{}", no_match);
    }
}

fn copy_collaborator(collab: &Collaborator) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let formatted = format!("{} <{}>", collab.name, collab.email);
    ctx.set_contents(formatted.to_owned()).unwrap();
    println!("{} copied!", formatted);
}

fn collaborators_by_slug(slug: &str) -> Vec<Collaborator> {
    let collabs = collaborators_hash();
    collabs
        .into_iter()
        .filter(|c| &c.slug == slug)
        .collect()
}

fn last_used_collaborator() -> Option<Collaborator> {
    let collabs = collaborators_hash();
    let matches: Vec<Collaborator> = collabs
        .into_iter()
        .filter(|c| &c.last_used == &true)
        .collect();

    if matches.len() > 0  {
        let collab = &matches[0];
        return Some(Collaborator::new(&collab.slug, &collab.name, &collab.email))
    } else {
        return None
    }
}

fn update_last_used(current: &Collaborator) {
    let mut existing_collabs = collaborators_hash();
    let mut new_last_used = Collaborator::new(&current.slug, &current.name, &current.email);
    new_last_used.last_used = true;

    let old_last_used: Collaborator;

    let last_used = last_used_collaborator();
    if last_used.is_some() {
        let collab = last_used.unwrap();
        let old_index = existing_collabs.iter().position(|r| r.slug == collab.slug).unwrap();
        existing_collabs.remove(old_index);
        old_last_used = Collaborator::new(&collab.slug, &collab.name, &collab.email);
        existing_collabs.push(old_last_used);
    }

    let index = existing_collabs.iter().position(|r| r.slug == new_last_used.slug).unwrap();
    existing_collabs.remove(index);
    existing_collabs.push(new_last_used);
    let to_write = serde_json::to_string(&existing_collabs).expect("something went wrong");

    let _ = fs::write(config_file(), &to_write);
}

fn collaborators_hash() -> Vec<Collaborator> {
    let file = File::open(config_file()).expect("can't find collaborators.json file");
    let reader = BufReader::new(file);

    serde_json::from_reader(reader).expect("JSON was not well-formatted")
}

fn assited_creation() -> Collaborator {
    let mut slug = String::new();
    let mut name = String::new();
    let mut email = String::new();

    println!("Enter a identifier for your git collaborator:");
    std::io::stdin().read_line(&mut slug).unwrap();

    println!("\n\nEnter collaborator's full name:");
    std::io::stdin().read_line(&mut name).unwrap();

    println!("\n\nEnter collaborator's email:");
    std::io::stdin().read_line(&mut email).unwrap();
    println!("\n\n");

    return Collaborator::new(&slug.trim(), &name.trim(), &email.trim());
}