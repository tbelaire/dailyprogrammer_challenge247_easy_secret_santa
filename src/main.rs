use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;



fn main() {
    let path = Path::new("input/test1.txt");
    if let Ok(assignment) = work(&path) {
        print_list(&assignment);
    } else {
        println!("There was an error");
    }
}

fn work(path: &Path) -> Result<Vec<String>, Box<std::error::Error>> {
    let file = BufReader::new(
        try!(File::open(path)));
    let mut people: Vec<Vec<String>> = vec![];

    for l in file.lines() {
        // Could have an error each time.
        let l = try!(l);
        let family = l.split(' ').map(|s|{s.to_string()}).collect();
        people.push(family);
    }

    let assignment = people.into_iter().flat_map(|family| {family}).collect();
    return Ok(assignment);

}


fn print_list(assignment: &Vec<String>) {
    for (giver, receiver) in assignment.iter().zip(
                             assignment.iter().cycle().skip(1)) {
        println!("{} -> {}", giver, receiver);
    }
}
