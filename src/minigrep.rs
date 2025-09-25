use std::env;
use std::fs;
use std::process;
use std::error::Error;

pub fn minigrep() {
    //get args and assign
    let args: Vec<String> = env::args().collect();
    //build config with file_path and query
    let config = Config::build(&args).unwrap_or_else(|err| {
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

fn find_instances(text: &mut String, phrase: &String) -> Result<(), Box<dyn Error>> {
    let phrase_bytes = phrase.as_bytes();
    let text_bytes = unsafe { text.as_bytes_mut() };

    if text_bytes.len() < phrase_bytes.len() {
        return Err("Phrase can not be longer than text.".into());
    }

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

struct Config {
    query: String,
    file_path: String
}
impl Config{
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("Use 2 arguments(search text and filename)");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
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
