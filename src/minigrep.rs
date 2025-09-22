use std::env;
use std::fs;
use std::process;
use std::error::Error;

pub fn minigrep() {
    //get args and assign
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In the file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1)
    }

    //loop through every chunk of words that is the same length as the text we are searching for and see if it matches
    //if searching for one word, loop through everything in contents and all caps every instance
    //if searching for multiple words, can loop through by length of the phrase, if you run across a portion of the phrase go to point where the words in that found portion matches the position of the search phrase and see if it matches
    //ex: phrase = "a big fat dog" currently looking at in file == "we had a big": you would skip over two words in the file so that "a big" are the first two words and see if it mathes "a big fat dog"
    //for phrase in contents
}

fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.file_path)?;

    println!("{file_contents}");

    Ok(())
}

struct Config {
    query: String,
    file_path: String
}
impl Config{
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
