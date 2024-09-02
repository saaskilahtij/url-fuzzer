use std::io;
use regex::Regex;

// TODO Create a working RegEx.

fn main() {

    let mut url = String::new();

    println!("Give me a URL to validate:");
    
    io::stdin()
        .read_line(&mut url)
        .expect("Failed to read the URL");

    if !validate_url(url)  {
        println!("False");
        return();
    }
    println!("True");
}

fn validate_url(url: String) -> bool {
    let schemes = "^https|^http|^ftp"
    let re = Regex::new("[{scheme}]://").unwrap();
    let schemes = vec!["http", "https", "ftp"];
    return re.find(url);
}
