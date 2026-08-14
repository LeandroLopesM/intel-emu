pub mod instructions;

use std::process::exit;

use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "asm.pest"]
struct AsmParser;

pub fn from_source(file: &String, source: String) {
    let pairs = AsmParser::parse(Rule::File, &source).unwrap_or_else(|e| {
        println!("{} {}", file, e);
        exit(1)
    });

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        // println!("Rule:    {:?}", pair.as_rule());
        // println!("Text:    {}", pair.as_str());
        // println!("Inner:    {:?}", pair.into_inner());

        // A pair can be converted to an iterator of the tokens which make it up:
        shred_pair(&pair, 0);
    }

    exit(0);
}

fn shred_pair(pair: &Pair<'_, Rule>, depth: usize) {
    println!(
        "{}Rule: {:?} \"{}\"",
        indent(depth),
        pair.as_rule(),
        pair.as_str()
    );

    for i in pair.clone().into_inner() {
        shred_pair(&i, depth + 1);
    }
}

fn indent(num: usize) -> String {
    ". ".repeat(num)
}
