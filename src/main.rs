use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;


// Le sigh, not stable...
// #[derive(Debug, Eq, PartialEq)]
struct Family(Vec<String>, bool);
impl Family {
    /// This is twice the len, +1 if you were first.
    fn weight(&self) -> usize {
        let l = self.0.len() * 2;
        if self.1 {
            l + 1
        } else {
            l
        }
    }
}

impl PartialEq for Family {
    fn eq(&self, b: &Family) -> bool {
        self.0.eq(&b.0)
    }
}
impl Eq for Family {}
// Sort familys by size.
impl Ord for Family {
    fn cmp(&self, b: &Family) -> Ordering {
        self.weight().cmp(&b.weight())
    }
}
impl PartialOrd for Family {
    fn partial_cmp(&self, b: &Family) -> Option<Ordering> {
        Some(self.cmp(b))
    }
}
impl Debug for Family {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        self.0.fmt(fmt)
    }
}

// *******************************************************************
fn main() {
    let path = Path::new("input/test2.txt");
    if let Ok(assignment) = work(&path) {
        print_list(&assignment);
    } else {
        println!("There was an error.");
        // Soo much better than panicing.... ;)
    }
}


fn work(path: &Path) -> Result<Vec<String>, Box<std::error::Error>> {
    let file = BufReader::new(
        try!(File::open(path)));
    let mut people: Vec<Family> = vec![];
    let mut num_people: usize = 0;

    for l in file.lines() {
        // Could have an error each time.
        let l = try!(l);
        let family: Vec<_>= l.split(' ').map(|s|{s.to_string()}).collect();
        num_people += family.len();
        assert!(family.len() > 0);
        // false as we haven't selected a first family yet.
        people.push(Family(family, false));
    }

    // The algorithm is as follows:
    // Make a heap of the familys, and pop the largest off, and remove
    // a member.
    // Then, pop off the next largest, remove a person, and push on the previous
    // one.
    let mut heap = BinaryHeap::from(people);

    let mut assignment: Vec<String> = Vec::with_capacity(num_people);
    let mut last_family = heap.pop().expect("At least one person required!");
    // These guys were first, do *not* let them be last as well.
    last_family.1 = true;

    assignment.push(last_family.0.pop().expect("At least one person in each family"));
    // println!("Assignment is {:?}", assignment);
    // println!("Heap is {:?}", heap);

    while let Some(mut next_family) = heap.pop() {
        assert!(next_family.0.len() > 0);
        assignment.push(next_family.0.pop().unwrap());
        // println!("Assignment is {:?}", assignment);
        // println!("Heap is {:?}", heap);

        if last_family.0.len() > 0 {
            heap.push(last_family);
        }
        last_family = next_family;
    }


    // let assignment = people.into_iter().flat_map(|family| {family.0}).collect();
    return Ok(assignment);

}


fn print_list(assignment: &Vec<String>) {
    println!("list is {:?}", assignment);
    for (giver, receiver) in assignment.iter().zip(
                             assignment.iter().cycle().skip(1)) {
        println!("{} -> {}", giver, receiver);
    }
}
