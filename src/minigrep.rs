use std::env;
use std::fs;
use std::process;
use std::error::Error;

pub fn minigrep() {
    //build config with env args
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    //catches all errors of read_and_run, run, and find instances
    if let Err(e) = read_and_run(&config) {
        println!("Application error: {e}");
        process::exit(1)
    }
}
//read file contents from fileand call run
fn read_and_run(config: &Config) -> Result<(), Box<dyn Error>> {
    let mut file_contents = fs::read_to_string(&config.file_path)?;

    run(&mut file_contents, &config.query)?;

    Ok(())
}
//run find_instances on text from file and search phrase, print out final result
fn run(text: &mut String, phrase: &String) -> Result<(), Box<dyn Error>> {
    find_instances(text, phrase)?;
    println!("{}", text);

    Ok(())
}
//check each u8 reference to make sure its the same when lowercase
fn ascii_eq_ignore_case_bytes(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter()
        .zip(b.iter())
        .all(|(&ac, &bc)| ac.to_ascii_lowercase() == bc.to_ascii_lowercase())
}
//breaks text and phrase into bytes and finds every instance where a portion of text matches phrase and makes it all caps
fn find_instances(text: &mut String, phrase: &String) -> Result<(), Box<dyn Error>> {
    let phrase_bytes = phrase.as_bytes();
    let text_bytes = unsafe { text.as_bytes_mut() };

    if text_bytes.len() < phrase_bytes.len() {
        return Err("Phrase can not be longer than text.".into());
    }
    //for every character necessary to look at take a chunk of text = to phrase length and compare it to phrase, all caps if =
    for i in 0..=text_bytes.len() - phrase_bytes.len() {
        if ascii_eq_ignore_case_bytes(&text_bytes[i..i + phrase_bytes.len()], phrase_bytes) {
            let start_ok = i == 0 || text_bytes[i - 1].is_ascii_whitespace();
            let end_ok = i + phrase_bytes.len() == text_bytes.len() ||
                text_bytes[i + phrase_bytes.len()].is_ascii_whitespace();

            if start_ok && end_ok {
                for b in &mut text_bytes[i..i + phrase_bytes.len()] {
                    if b.is_ascii_lowercase() {
                        *b = b.to_ascii_uppercase();
                    }
                }
            }
        }
    }
    Ok(())
}
//struct for the file path and search string
struct Config {
    query: String,
    file_path: String
}
impl Config{
    //iterate through args and assign to vars to build Config
    //pass Iterator as arg because env.args() returns iterator
    fn build(mut args: impl Iterator<Item = String>,) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("No query string")
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("No file path")
        };

        Ok(Config {query, file_path})
    }
}

#[cfg(test)]
mod minigrep_test {
    use super::*;

    #[test]
    fn base_test() {
        let mut text = String::from("This is this test for this text.");
        let query = String::from("this");

        let result = run(&mut text, &query);
        assert!(result.is_ok());
    }
}
