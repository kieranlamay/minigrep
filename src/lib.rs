use std::env;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

pub fn build_config(mut args: env::Args) -> Result<Config, String> {
    args.next(); // skip program name

    let Some(query) = args.next() else {
        return Err(String::from("Missing query"));
    };

    let Some(filename) = args.next() else {
        return Err(String::from("Missing filename"));
    };
    if args.next().is_some() {
        return Err(String::from("Too many arguments"));
    }

    let case_insensitive: bool = env::var("CASE_INSENSITIVE").is_ok();

    Ok(Config {
        query,
        filename,
        case_insensitive,
    })
}

pub fn run(config: &Config) -> Result<(), String> {
    let contents = fs::read_to_string(&config.filename);
    match contents {
        Ok(text) => {
            let matchlines: Vec<&str> = match config.case_insensitive {
                true => case_insensitive_search(&config.query, &text),
                false => search(&config.query, &text),
            };
            for line in matchlines {
                println!("{line}");
            }
            Ok(())
        }
        Err(_) => Err(String::from("Unable to read from file")),
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

fn case_insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&str> = Vec::new();
    let lowercase_query: String = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&lowercase_query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "Two roads diverged in a yellow wood,\n
And sorry I could not travel both\n
And be one traveler, long I stood\n
And looked down one as far as I could\n
To where it bent in the undergrowth;\n
\n
Then took the other, as just as fair,\n
And having perhaps the better claim,\n
Because it was grassy and wanted wear;\n
Though as for that the passing there\n
Had worn them really about the same,\n
\n
And both that morning equally lay\n
In leaves no step had trodden black.\n
Oh, I kept the first for another day!\n
Yet knowing how way leads on to way,\n
I doubted if I should ever come back.\n
\n
I shall be telling this with a sigh\n
Somewhere ages and ages hence:\n
Two roads diverged in a wood, and I—\n
I took the one less traveled by,\n
And that has made all the difference.\n
";

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn match_line_sensitive() {
        assert_eq!(search("Hello", "Hello world!"), vec!["Hello world!"]);
    }

    #[test]
    fn match_lines_sensitive() {
        assert_eq!(
            search("And", CONTENT),
            vec![
                "And sorry I could not travel both",
                "And be one traveler, long I stood",
                "And looked down one as far as I could",
                "And having perhaps the better claim,",
                "And both that morning equally lay",
                "And that has made all the difference.",
            ]
        )
    }

    #[test]
    fn not_match_sensitive() {
        assert_eq!(search("hello", "Hello world"), Vec::<&str>::new());
    }

    #[test]
    fn match_line_insensitive() {
        assert_eq!(
            case_insensitive_search("hElLo", "Hello world!"),
            vec!["Hello world!"]
        );
    }

    #[test]
    fn match_lines_insensitive() {
        assert_eq!(
            case_insensitive_search("aNd", CONTENT),
            vec![
                "And sorry I could not travel both",
                "And be one traveler, long I stood",
                "And looked down one as far as I could",
                "And having perhaps the better claim,",
                "Because it was grassy and wanted wear;",
                "And both that morning equally lay",
                "Somewhere ages and ages hence:",
                "Two roads diverged in a wood, and I—",
                "And that has made all the difference.",
            ]
        )
    }

    #[test]
    fn not_match_insensitive() {
        assert_eq!(case_insensitive_search("xyz", CONTENT), Vec::<&str>::new());
    }
}
