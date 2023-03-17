use mdbook::errors::Result as MdbookResult;
use regex::Regex;

const ARGUMENT_REGEX: &str = r#"(?:"(?:\\"|[^"])+"|'(?:\\'|[^'])++'|[^"'\s]+)(?:\s|$)"#;

pub struct EnvInfo {
    pub name: String,
    pub args: Vec<String>,
}

impl EnvInfo {
    pub fn parse(info_string: &str) -> MdbookResult<EnvInfo> {
         match info_string.split_once(' ') {
            Some((name, rest)) => {
                Ok(EnvInfo {
                    name: String::from(name),
                    args: parse_arguments(rest),
                })
            },
            None => Ok(EnvInfo {
                name: String::from(info_string),
                args: vec![],
            })
         }
    }
}

fn parse_arguments(whole: &str) -> Vec<String> {
    Regex::new(ARGUMENT_REGEX).unwrap()
        .find_iter(whole)
        .map(|m| whole[m.start()..m.end()].trim())
        .map(|x| String::from(match &x[0..1].as_ref() {
            &"\"" | &"'" => &x[1..x.len()-1],
            _ => x,
        }))
        .collect()
}