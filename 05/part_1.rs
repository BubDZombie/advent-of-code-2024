use std::collections::HashMap;
//use std::collections::hash_map::Entry::Occupied;
//use std::collections::hash_map::Entry::Vacant;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut rules: HashMap<String, HashSet<String>> = HashMap::new();
    let mut updates: Vec<Vec<String>> = Vec::new();
    for line in reader.lines() {
        let l = line.unwrap();
        if l.contains("|") {
            let mut parts = l.split('|');
            let key = parts.next().unwrap().to_string();
            let val = parts.next().unwrap().to_string();
            println!("{} {}", key, val);
            let entry = rules.entry(key).or_insert(HashSet::new());
            entry.insert(val);
        } else if l.contains(",") {
            let pages = l.split(',');
            updates.push(pages.map(|page| page.to_string()).collect());
        }
    }
    let mut good_sum = 0i32;
    let mut bad_sum = 0i32;
    for mut update in updates {
        println!("Processing {:?}", update);
        let mut seen = HashSet::new();
        let mut broken = false;
        let mut i = 0;
        while i < update.len() {
            let page = &update[i];
            seen.insert(page.clone());
            let mut should_increment = true;
            if rules.get(page).is_some() {
                for bad_page in &rules[page] {
                    if seen.contains(bad_page) {
                        broken = true;
                        let bad_index = update.iter().position(|page| page == bad_page).unwrap();
                        update.swap(i, bad_index);
                        println!("Swapped {} and {}: {:?}", update[i], update[bad_index], update);
                        i = 0;
                        seen = HashSet::new();
                        should_increment = false;
                        break;
                    }
                }
            }
            if should_increment {
                i += 1;
            }
        }
        if broken {
            bad_sum += update[update.len()/2].parse::<i32>().unwrap();
            println!("Bad list {:?} {}", update, bad_sum);
        } else {
            good_sum += update[update.len()/2].parse::<i32>().unwrap();
            println!("Good list {:?} {}", update, good_sum);
        }
    }
    Ok(())
}
