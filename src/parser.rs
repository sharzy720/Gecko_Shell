extern crate pest;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ArgParser;

pub fn parse(to_parse: String) -> Vec<String> {
    // Parsing the input string via the `line` rule in grammar.pest
    let parsed = ArgParser::parse(Rule::line, &to_parse)
        .expect("Failed to parse")
        .next()
        .unwrap();

    let mut tokens: Vec<String> = vec![];

    // Gets `line`

    for line in parsed.into_inner() {
        // `line` is either `quoted` or `commands`
        match line.as_rule() {
            Rule::quoted => {
                // Trim the quotation marks off of the string
                let token = line.as_str();
                let token = &token[1..token.len() - 1];
                tokens.push(token.to_string());
            }
            Rule::commands => {
                // Get each `command` present
                for command in line.into_inner() {
                    tokens.push(command.as_str().to_string());
                }
            }
            _ => {
                unreachable!();
            }
        }
    }

    tokens
}
