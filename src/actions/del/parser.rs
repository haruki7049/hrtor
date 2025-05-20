use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "actions/del/args.pest"]
struct ArgsParser;

#[derive(Debug)]
pub struct Args<'a> {
    pub first_arg: &'a str,
    pub last_arg: &'a str,
}

fn parse_pair(pair: Pair<Rule>) -> anyhow::Result<Args> {
    match pair.as_rule() {
        Rule::EOI | Rule::PUNCT_WORD | Rule::first_line | Rule::last_line => unreachable!(),
        Rule::ARGUMENTS => {
            let mut rule = pair.into_inner();
            let first_arg = rule.next().unwrap();
            let last_arg = rule.next().unwrap();

            Ok(Args {
                first_arg: first_arg.as_span().as_str(),
                last_arg: last_arg.as_span().as_str(),
            })
        }
    }
}

pub fn parse(s: &str) -> anyhow::Result<Args> {
    let mut pairs = ArgsParser::parse(Rule::ARGUMENTS, s)?;

    parse_pair(pairs.next().unwrap())
}
