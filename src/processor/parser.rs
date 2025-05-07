use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "processor/expr.pest"]
pub struct HrtorParser;

#[derive(Debug)]
pub struct Expression {
    pub cmd: Command,
}

#[derive(Debug, PartialEq)]
pub enum Command {
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
                "add" => return Ok(Expression { cmd: Command::Add }),
                "delete_all" => {
                    return Ok(Expression {
                        cmd: Command::DeleteAll,
                    });
                }
                "exit" => return Ok(Expression { cmd: Command::Exit }),
                "print" => {
                    return Ok(Expression {
                        cmd: Command::Print,
                    });
                }
                "write" => {
                    return Ok(Expression {
                        cmd: Command::Write,
                    });
                }
                _ => unreachable!(),
            }
        }
    }
}

pub fn parse(s: &str) -> anyhow::Result<Expression> {
    let mut pairs = HrtorParser::parse(Rule::expr, s)?;

    Ok(parse_pair(pairs.next().unwrap())?)
}
