use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

pub struct Config {
    pub path: String,
    pub top: usize,
    pub min_len: usize
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next(); // skip the program name

        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("usage: wordtally <path> [top-n]"),
        };

        let top = match args.next() {
            Some(value) => value.parse().map_err(|_| "top-n must be number")?,
            None => 10
        };

        let min_len = std::env::var("MIN_LEN")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1);

        Ok(Config { path, top, min_len })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut totals = HashMap::new();
    tally_path(Path::new(&config.path), config.min_len, &mut totals)?;
    for (word, count) in rank(totals, config.top) {
         println!("{count:>6}  {word}");
    }
    Ok(())
}

fn count_words(text: &str, min_len: usize) -> HashMap<String, usize> {
    text.split_whitespace()
        .map(normalize)
        .filter(|word| word.chars().count() >= min_len)
        .fold(HashMap::new(), |mut counts, word| {
            *counts.entry(word).or_insert(0) += 1;
            counts
        })
}

fn normalize(raw: &str) -> String {
    raw.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect()
}

fn rank(counts: HashMap<String, usize>, top: usize) -> Vec<(String, usize)> {
    let mut pairs: Vec<(String, usize)> = counts.into_iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    pairs.into_iter().take(top).collect()
}

fn tally_path(
    path: &Path,
    min_len: usize,
    totals: &mut HashMap<String, usize>,
) -> std::io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            tally_path(&entry?.path(), min_len, totals)?;
        }
    } else if let Ok(text) = fs::read_to_string(path) {
        for (word, count) in count_words(&text, min_len) {
            *totals.entry(word).or_insert(0) += count;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_and_lowercases_words() {
        let counts = count_words("The cat sat. The CAT ran!", 1);
        assert_eq!(counts.get("the"), Some(&2));
        assert_eq!(counts.get("cat"), Some(&2));
    }

    #[test]
    fn rank_orders_by_count_and_limits() {
        let mut counts = HashMap::new();
        counts.insert(String::from("the"), 5);
        counts.insert(String::from("cat"), 2);
        counts.insert(String::from("sat"), 1);
        assert_eq!(
            rank(counts, 2),
            vec![(String::from("the"), 5), (String::from("cat"), 2)]
        );
    }
}