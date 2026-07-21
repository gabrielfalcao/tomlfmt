use crate::parser::toml::Rule;
use crate::{Error, Result, Source, Span, SpanPosition};
use pest::Parser;
use std::borrow::Cow;
use pest::iterators::Pairs;
pub mod toml {
    #[derive(Parser)]
    #[grammar = "grammar.pest"]
    pub struct TomlParser;
}

pub fn parse_source<'a>(input: &'a str) -> Result<Pairs<'a, Rule>> {
    let source_info = Source {
        source: Cow::from(input),
        filename: None,
    };
    let mut pairs = toml::TomlParser::parse(Rule::toml, input)?;
    Ok(pairs)
}

//pub fn map_pairs_to_list<'a>(pairs: Pairs<'a, Rule>) -> Value<'a> {
//    pairs.map(|pair| pair_to_value(pair)).collect()
//}
//pub fn pair_to_value<'a>(pair: Pair<'a, Rule>) -> Value<'a> {
//    match pair.as_rule() {
//        Rule::float =>
//            Value::float(f64::from_str(pair.as_span().as_str()).expect("float")),
//        Rule::integer =>
//            Value::integer(i64::from_str(pair.as_span().as_str()).expect("integer")),
//        Rule::string => Value::string(Cow::from(pair.as_span().as_str())),
//        Rule::symbol => Value::symbol(Cow::from(pair.as_span().as_str())),
//        Rule::quoted_symbol => {
//            let mut pairs = pair.clone().into_inner();
//            pairs.next().expect("quote");
//            let symbol = pairs.next().expect("symbol");
//            Value::quoted_symbol(symbol.as_span().as_str())
//        },
//        Rule::t => Value::T,
//        Rule::unsigned => Value::unsigned_integer(
//            u32::from_str(pair.as_span().as_str()).expect("unsigned integer"),
//        ),
//        Rule::value => pair_to_value(pair.clone().into_inner().next().expect("value")),
//        Rule::sexpr => {
//            let mut items = Cell::nil();
//            let mut pairs = pair.clone().into_inner();
//            let mut quoted = false;
//            loop {
//                if let Some(pair) = pairs.peek() {
//                    if pair.as_rule() == Rule::close_paren {
//                        break;
//                    }
//                }
//                let pair = pairs.next().expect("quote, open_paren or item");
//                match pair.as_rule() {
//                    Rule::quote => {
//                        quoted = true;
//                    },
//                    Rule::open_paren => continue,
//                    Rule::close_paren => continue,
//                    _ => {
//                        items.push_value(pair_to_value(pair));
//                        continue;
//                    },
//                }
//            }
//            pairs.next().expect("close_paren");
//            let value = Value::from_iter(items.into_iter());
//            if quoted {
//                value.quote()
//            } else {
//                value
//            }
//        },
//        Rule::nil => Value::nil(),
//        _ => unexpected!(pair),
//    }
//}
//
