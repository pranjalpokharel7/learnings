// I need to build a mini tool that will consume git commit message
// and reject the commit if it doesn't start with feat:<space> or fix:<space>

use std::io::{stdin, BufRead}; // need to bring BufRead into scope so that we can access the lines() method

const ERROR_MSG: &str = "I have commitment issues!";

fn main() {
    analyze_commit_msg();
}

fn analyze_msg(msg: String) {
    let parts: Vec<&str> = msg.split(":").collect();
    if parts.len() == 1 || (parts[0] != "feat" && parts[0] != "fix") {
        panic!("{}", ERROR_MSG);
    }
}

// it seems I can define functions after main as well
fn analyze_commit_msg() {
    let stdin = stdin(); // varaible stdin is shadowed, it is no longer a function from now
    for line in stdin.lock().lines() {
        // commit-msg hook provides the commit message as a part of the first argument
        return match line {
            Ok(msg) => analyze_msg(msg),
            Err(err) => panic!("{:?}", err),
        }
    }
}
