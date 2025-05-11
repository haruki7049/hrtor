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
        Rule::EOI | Rule::action | Rule::arguments | Rule::PUNCT_WORD => unreachable!(),
        Rule::expr => {
            let mut rule = pair.into_inner();
            let action = rule.next().unwrap();
            let arguments: String = rule.next().unwrap().as_span().as_str().to_string();

            match action.as_span().as_str() {
                "add" => Ok(Expression {
                    action: Action::Add,
                    arguments,
                }),
                "delete_all" => Ok(Expression {
                    action: Action::DeleteAll,
                    arguments,
                }),
                "exit" => Ok(Expression {
                    action: Action::Exit,
                    arguments,
                }),
                "print" => Ok(Expression {
                    action: Action::Print,
                    arguments,
                }),
                "write" => Ok(Expression {
                    action: Action::Write,
                    arguments,
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
