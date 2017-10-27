#![feature(exact_size_is_empty)]
use std::io::{self, Read};
use std::process::Command;
use std::env;

mod branch;
mod token;

use branch::Branch;

fn main() {
    let entry = parse_args();
    if let Ok(branch) = Branch::new(entry) {
        let branch_name = branch.parse();
        match create_git_branch(&branch_name) {
            Ok(_) => return,
            Err(e) => print!("{}", e),
        }
    }
}

fn parse_args() -> String {
    let mut entry = String::new();
    if env::args().is_empty() {
        match io::stdin().read_to_string(&mut entry) {
            Ok(_) => {},
            Err(e) => panic!(e),
        };
    } else {
        entry = env::args().skip(1).collect::<Vec<String>>().join(" ");
    }
    entry
}

fn create_git_branch(name: &str) -> Result<(), String> {
    let output = Command::new("git")
            .arg("branch")
            .arg(name)
            .output().expect("Error calling git");
    if output.status.success() {
        Ok(println!("branch {} has been created", name))
    } else {
        let err = String::from_utf8(output.stderr).unwrap();
        Err(err)
    }
}

#[test]
fn test_normal_name() {
    let name = "BRANCH-233 some useless branch".to_owned();
    if let Ok(b) = Branch::new(name) {
        assert_eq!(b.parse(), "BRANCH_233__some_useless_branch")
    };
}

#[test]
fn test_weird_name() {
    let name = "BRANCH-233 23s0me ess-branch".to_owned();
    if let Ok(b) = Branch::new(name) {
        assert_eq!(b.parse(), "BRANCH_233__23s0me_ess_branch")
    };
}
