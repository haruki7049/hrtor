use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "processor/expr.pest"]
pub struct HrtorParser;

#[derive(Debug)]
pub struct Expression<'i> {
    pub action: Action,

    /// # Arguments: &'static str
    /// This value is parsed by each actions
    pub arguments: &'i str,
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Add,
    DeleteAll,
    Exit,
    Git,
    Grep,
    Open,
    Print,
    Write,
    Tutorial,
}

fn parse_pair(pair: Pair<Rule>) -> anyhow::Result<Expression> {
    match pair.as_rule() {
        Rule::EOI | Rule::action | Rule::arguments | Rule::PUNCT_WORD => unreachable!(),
        Rule::expr => {
            let mut rule = pair.into_inner();
            let action = rule.next().unwrap();
            let arguments = rule.next().unwrap();

            match action.as_span().as_str() {
                "add" => Ok(Expression {
                    action: Action::Add,
                    arguments: arguments.as_span().as_str(),
                }),
                "delete_all" => Ok(Expression {
                    action: Action::DeleteAll,
                    arguments: arguments.as_span().as_str(),
                }),
                "exit" => Ok(Expression {
                    action: Action::Exit,
                    arguments: arguments.as_span().as_str(),
                }),
                "git" => Ok(Expression {
                    action: Action::Git,
                    arguments: arguments.as_span().as_str(),
                }),
                "grep" => Ok(Expression {
                    action: Action::Grep,
                    arguments: arguments.as_span().as_str(),
                }),
                "open" => Ok(Expression {
                    action: Action::Open,
                    arguments: arguments.as_span().as_str(),
                }),
                "print" => Ok(Expression {
                    action: Action::Print,
                    arguments: arguments.as_span().as_str(),
                }),
                "write" => Ok(Expression {
                    action: Action::Write,
                    arguments: arguments.as_span().as_str(),
                }),
                "tutorial" => Ok(Expression {
                    action: Action::Tutorial,
                    arguments: arguments.as_span().as_str(),
                }),
                s => anyhow::bail!("ACTION_LOADING_ERROR: Unknown action's name, {}", s),
            }
        }
    }
}

pub fn parse(s: &str) -> anyhow::Result<Expression> {
    let mut pairs = HrtorParser::parse(Rule::expr, s)?;

    parse_pair(pairs.next().unwrap())
}
