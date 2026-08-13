pub mod instructions;

use std::process::exit;

use pest_derive::Parser;
use pest::{Parser, iterators::{Pair, Pairs}};

#[derive(Parser)]
#[grammar = "asm.pest"]
struct AsmParser;

pub fn from_source(source: String) {
    let pairs = AsmParser::parse(Rule::Instruction, &source).unwrap_or_else(|e| panic!("{}", e));

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        
    }

    exit(0);

}