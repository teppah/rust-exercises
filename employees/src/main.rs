use std::io;
use std::collections::HashMap;

fn main() {
    let mut workers: HashMap<String, Vec<String>> = HashMap::new();
    let stdin = io::stdin();
    println!("-- initialized program");
    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).expect("failed to read line");
        let processed = input.trim().to_lowercase();
        let mut words = processed.split_whitespace();
        match words.next() {
            Some("add") => {
                let person_name = match words.next() {
                    Some(t) => t,
                    None => {
                        eprintln!("-- ERROR: missing department name");
                        continue;
                    }
                };
                let department_name = match words.next() {
                    Some(t) => t,
                    None => {
                        eprintln!("-- ERROR: missing person name");
                        continue;
                    }
                };
                let department =
                    workers
                        .entry(department_name.to_lowercase())
                        .or_insert(Vec::new());
                department.push(String::from(person_name));
                department.sort();
                println!("-- inserted person \"{}\" into department \"{}\"", person_name, department_name);
            }
            Some("list") => {
                match words.next() {
                    Some("all") => {
                        for (program, people) in workers.iter() {
                            println!("-- Workers in {}", program);
                            for name in people {
                                println!("-- -- {}", name);
                            }
                        }
                    }
                    Some(p) => {
                        for (program, people) in workers.iter() {
                            if program == p {
                                println!("-- Workers in {}", program);
                                for name in people {
                                    println!("-- -- {}", name);
                                }
                            }
                        }
                    }
                    None => {}
                }
            }
            Some("quit") => {
                println!("-- quitting...");
                break;
            }
            Some(_) => println!("-- invalid option, please retry"),
            None => {
                continue;
            }
        }
    }
}
