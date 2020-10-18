/// Explanation of module system
use wildmatch::WildMatch;

/// Can also be defined in another file called moderator.rs.
/// mod moderator;
mod moderator {
    use std::io;

    pub fn ask_question() -> String {
        println!("What did you do today?");
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).unwrap();
        // Same as ¨return buffer;¨
        buffer
    }
}

fn main() {
    let matcher = WildMatch::new("*rust*");
    let result = moderator::ask_question();
    if matcher.is_match(&result) {
        println!("Something with rust? Great!.");
    } else {
        println!("Ever heard of rust? Try it out.");
    }
}
