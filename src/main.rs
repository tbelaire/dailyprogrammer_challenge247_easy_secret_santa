use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;


#[derive(Debug, Eq, PartialEq)]
struct Family {
    people: Vec<String>,
    first: bool,
}

impl Family {
    pub fn new<T>(people: Vec<T>) -> Self
        where T: Into<String>
    {
        Family {
            people: people.into_iter().map(Into::into).collect(),
            first: false,
        }
    }
    pub fn new2<V, T>(people: V) -> Self
        where V: Into<Vec<T>>,
              T: Into<String>
    {
        Family {
            people: people.into().into_iter().map(Into::into).collect(),
            first: false,
        }
    }
}

// Sort families by size, then break ties so that the family with
// first set has a fractionally larger size.
// This avoids a family being both first and last.
impl Ord for Family {
    fn cmp(&self, b: &Family) -> Ordering {
        match self.people.cmp(&b.people) {
            Ordering::Equal => self.first.cmp(&b.first),
            other => other,
        }
    }
}
impl PartialOrd for Family {
    fn partial_cmp(&self, b: &Family) -> Option<Ordering> {
        Some(self.cmp(b))
    }
}

// *******************************************************************
fn main() {
    let path = Path::new("input/test2.txt");
    let people = match load_family(&path) {
        Ok(people) => people,
        Err(error) => {
            println!("There was an error loading the families: {}",
                     error.description());
            return;
        }
    };

    if let Some(assignment) = find_assignment(people) {
        print_list(&assignment);
    } else {
        println!("The list of people had no valid solution");
    }
}


fn load_family(path: &Path) -> Result<Vec<Family>, Box<std::error::Error>> {
    let file = try!(File::open(path));
    let file = BufReader::new(file);
    let mut people: Vec<Family> = vec![];

    for l in file.lines() {
        // Could have an error each time.
        let l = try!(l);
        let family: Vec<_> = l.split(' ').map(|s| s.to_owned()).collect();

        // Skip blank lines.
        if family.len() > 0 {
            // first: false as we haven't selected a first family yet.
            people.push(Family {
                people: family,
                first: false,
            });
        }
    }
    Ok(people)
}

// The algorithm is as follows:
// Make a heap of the families, and pop the largest off, and remove a member.
// Then, pop off the next largest, remove a person, and push on the previous
// family.
// Thus, at each step, we avoid taking from the same family twice in a row,
// as it hasn't been re-added to the heap yet.  At the same time, this focuses
// on finding matches for people in the largest families first, to avoid getting
// backed into a corner later.
//
// It's possible that no assignment exists that satisfies all the constraints,
// in which case this will return None.
fn find_assignment(people: Vec<Family>) -> Option<Vec<String>> {

    // If we have no people, then just stop.
    if people.is_empty() {
        return None;
    }

    let mut heap = BinaryHeap::from(people);

    let mut assignment: Vec<String> = Vec::new();
    let mut last_family = heap.pop().unwrap(); // We know there is at least one.

    // These guys were first, do *not* let them be last as well.
    // This gives them a boost in priority in the heap.
    last_family.first = true;

    assignment.push(last_family.people
                               .pop()
                               .expect("At least one person in each family"));

    while let Some(mut next_family) = heap.pop() {
        assignment.push(next_family.people
                                   .pop()
                                   .expect("Somehow an empty family is in \
                                            the heap"));

        if last_family.people.len() > 0 {
            heap.push(last_family);
        }
        last_family = next_family;
    }
    if last_family.people.len() > 0 {
        // We failed to assign everyone in this family before we ran out of
        // other families, so the initial configuration was not feasible.
        None
    } else {
        Some(assignment)
    }
}


fn print_list(assignment: &[String]) {
    for (giver, receiver) in assignment.iter()
                                       .zip(assignment.iter().cycle().skip(1)) {
        println!("{} -> {}", giver, receiver);
    }
}

#[test]
fn test_infeasible() {
    let people = vec![Family {
                          people: vec!["Appa".to_owned(),
                                       "Alph".to_owned(),
                                       "Alex".to_owned()],
                          first: false,
                      },
                      Family {
                          people: vec!["Bob".to_owned()],
                          first: false,
                      }];
    assert_eq!(find_assignment(people), None);
}

#[test]
fn test_family_constructor() {
    let family1 = Family {
        people: vec!["Appa".to_owned(), "Alph".to_owned(), "Alex".to_owned()],
        first: false,
    };
    let family2 = Family::new(vec!["Appa", "Alph", "Alex"]);
    assert_eq!(family1, family2);
    let family3 = Family::new2(&["Appa", "Alph", "Alex"][..]);
    assert_eq!(family1, family3);
}
