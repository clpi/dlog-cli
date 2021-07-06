use pest_derive::Parser;
use pest::RuleType;

pub trait DlParser {
    type Rule: RuleType;
}

#[derive(Parser, Default)]
#[grammar = "grammar/grammar.pest"]
pub struct DlBaseParser;

impl DlParser for DlBaseParser {
    type Rule = Rule;

}

impl DlBaseParser {
    pub fn parse_input() -> String {
        let parser = Self;

        String::new()
    }
}
