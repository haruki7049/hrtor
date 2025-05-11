use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "processor/expr.pest"]
pub struct HrtorParser;

#[derive(Debug)]
pub struct Expression {
    pub action: Action,

    /// # Arguments: String
    /// This value is parsed by each actions
    pub arguments: String,
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Add,
    DeleteAll,
    Exit,
    Print,
    Write,
}

fn parse_pair(pair: Pair<Rule>) -> anyhow::Result<Expression> {
    match pair.as_rule() {
        Rule::EOI => panic!("EOI detected by parser"),
        Rule::command => unreachable!(),
        Rule::expr => {
            let mut rule = pair.into_inner();
            let command = rule.next().unwrap();

            // Now command has a str as "write"

            match command.as_span().as_str() {
                "add" => Ok(Expression {
                    action: Action::Add,
                    arguments: String::new(),
                }),
                "delete_all" => Ok(Expression {
                    action: Action::DeleteAll,
                    arguments: String::new(),
                }),
                "exit" => Ok(Expression {
                    action: Action::Exit,
                    arguments: String::new(),
                }),
                "print" => Ok(Expression {
                    action: Action::Print,
                    arguments: String::new(),
                }),
                "write" => Ok(Expression {
                    action: Action::Write,
                    arguments: String::new(),
                }),
                _ => unreachable!(),
            }
        }
    }
}

pub fn parse(s: &str) -> anyhow::Result<Expression> {
    let mut pairs = HrtorParser::parse(Rule::expr, s)?;

    parse_pair(pairs.next().unwrap())
}
