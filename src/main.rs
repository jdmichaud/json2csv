extern crate serde_json;
extern crate structopt;

use serde_json::{Deserializer, Value};
use std::io::{Cursor, BufRead, stdin};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Options {
  /// The keys to extract from the json objects to the csv line.
  /// If not provided, use keys of the first object. (ex: -k id,name,address)
  #[structopt(short="k", long, multiple=true, value_delimiter=",")]
  include_keys: Option<Vec<String>>,
  /// Exclude keys. (ex: -e confidential_key)
  #[structopt(short="e", long, multiple=true, value_delimiter=",")]
  exclude_keys: Option<Vec<String>>,
  /// Separator
  #[structopt(short, long, default_value=",")]
  separator: String,
}

// Convert the json value to a csv line
fn line2csv(input: Value, keys: &Vec<String>, separator: &str) -> String {
  keys
    .iter()
    .map(|key| {
      match input.get(key) {
        Some(value) => format!("{}", value),
        None => "".to_string()
      }
    })
    .collect::<Vec<String>>()
    .join(separator)
}

// Read the buffered reader and call back for the header line and every entry in the stream.
fn convert<T: BufRead, F: FnMut(String)>(reader: T, options: Options, mut callback: F) {
  let mut stream = Deserializer::from_reader(reader).into_iter::<Value>();
  // Get the first element to create the header
  let first = match stream.next() {
    Some(result) => match result {
      Ok(result) => result,
      Err(_error) => return (),
    },
    None => return (), // EOF before any record
  };

  let keys: Vec<String> = match options.include_keys {
    Some(keys) => {
      keys
    }
    None => {
      // Collect the keys of the first object received
      let keys =  first.as_object()
        .expect("first entry must be an object")
        .keys()
        .map(|s| String::from(s))
        .collect::<Vec<String>>();
      keys
    },
  };

  let filtered_key = match options.exclude_keys {
    Some(ek) => keys.into_iter().filter(|key| ek.iter().any(|i| i != key)).collect::<Vec<String>>(),
    None => keys,
  };

  callback(filtered_key.join(&options.separator));
  callback(line2csv(first, &filtered_key, &options.separator));
  for value in stream {
    callback(line2csv(value.unwrap(), &filtered_key, &options.separator));
  }
}

#[test]
fn convert_test() {
  let mut result: Vec<String> = Vec::new();
  // Extract 2 lines with not option
  convert(
      Cursor::new(r#"{"a": 3, "b": 4, "c": 5} {"a": 6, "b": 7, "c": 8}"#),
      Options { include_keys: None, exclude_keys: None, separator: ",".to_string() },
      |output| result.push(output));
  assert_eq!(result, &["a,b,c", "3,4,5", "6,7,8"]);
  // Use a deferent separator
  result.clear();
  convert(
      Cursor::new(r#"{"a": 3, "b": 4, "c": 5} {"a": 6, "b": 7, "c": 8}"#),
      Options {
        include_keys: None,
        exclude_keys: None,
        separator: "@".to_string(),
      },
      |output| result.push(output));
  assert_eq!(result, &["a@b@c", "3@4@5", "6@7@8"]);
  // Restrict to some fields with missing field
  result.clear();
  convert(
      Cursor::new(r#"{"a": 3, "b": 4, "c": 5} {"a": 6, "b": 7, "c": 8} {"b": 9, "c": 0}"#),
      Options {
        include_keys: Some(vec!["a".to_string(), "c".to_string()]),
        exclude_keys: None,
        separator: ",".to_string(),
      },
      |output| result.push(output));
  assert_eq!(result, &["a,c", "3,5", "6,8", ",0"]);
  // Exclude keys from the first object read
  result.clear();
  convert(
      Cursor::new(r#"{"a": 3, "b": 4, "c": 5} {"a": 6, "b": 7, "c": 8} {"b": 9, "c": 0}"#),
      Options {
        include_keys: None,
        exclude_keys: Some(vec!["b".to_string()]),
        separator: ",".to_string(),
      },
      |output| result.push(output));
  assert_eq!(result, &["a,c", "3,5", "6,8", ",0"]);
}

fn main() {
  let opt = Options::from_args();
  convert(stdin().lock(), opt, |output| println!("{}", output));
}
