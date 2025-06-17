use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "expr.pest"]
struct HrtorParser;

#[derive(Debug)]
pub struct Expression<'i> {
    pub action: &'i str,

    /// # Arguments: &'static str
    /// This value is parsed by each actions
    pub arguments: &'i str,
}

fn parse_pair(pair: Pair<Rule>) -> anyhow::Result<Expression> {
    match pair.as_rule() {
        Rule::EOI | Rule::action | Rule::arguments | Rule::PUNCT_WORD => unreachable!(),
        Rule::expr => {
            let mut rule = pair.into_inner();
            let action = rule.next().unwrap();
            let arguments = rule.next().unwrap();

            Ok(Expression {
                action: action.as_span().as_str(),
                arguments: arguments.as_span().as_str(),
            })
        }
    }
}

pub fn parse(s: &str) -> anyhow::Result<Expression> {
    let mut pairs = HrtorParser::parse(Rule::expr, s)?;

    parse_pair(pairs.next().unwrap())
}
