use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::env;
use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let re = Regex::new(r"(?<u>\w+)\s?--\s?(?<v>\w+);").unwrap();

    let args: Vec<String> = env::args().collect();
    
    let mut vertices = HashSet::new();
    let mut edges = HashSet::new();

    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines(&args[1]) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let Some(caps) = re.captures(&line) else {
                continue;
            };
            vertices.insert(format!("({}:PERSON {{name:\"{}\"}})", &caps["u"], &caps["u"]));
            vertices.insert(format!("({}:PERSON {{name:\"{}\"}})", &caps["v"], &caps["v"]));
            edges.insert(format!("({})-[:CONN]->({})", &caps["u"], &caps["v"]));
            edges.insert(format!("({})-[:CONN]->({})", &caps["v"], &caps["u"]));

            //println!("Edge::new(\"{}\", \"{}\"),", &caps["u"], &caps["v"]);
        }
    }

    println!("CREATE\n  {},", vertices.iter().join(",\n  "));
    println!("  {};", edges.iter().join(",\n  "));
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
