// I need to build a mini tool that will consume git commit message
// and reject the commit if it doesn't start with feat: or fix:

use std::env;
use std::fs;
use std::process::exit;

const COMMITMENT_ISSUE: &str = "I have commitment issues!";

fn main() {
    analyze_commit_msg();
}

fn analyze_msg(msg: String) {
    let parts: Vec<&str> = msg.split(":").collect();
    if parts.len() == 1 || (parts[0] != "feat" && parts[0] != "fix") {
        println!("{}", COMMITMENT_ISSUE);
        exit(1);
    }
}

fn analyze_commit_msg() {
    // commit-msg hook receives path to the temporary file that holds the commit message as CLI argument
    let commit_msg_file = env::args().nth(1).unwrap();
    let commit_msg = fs::read_to_string(commit_msg_file)
        .expect("File doesn't exist. Expected: commit message file path.");
    return analyze_msg(commit_msg);
}
