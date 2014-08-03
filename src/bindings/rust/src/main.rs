#![feature(globs)]

extern crate libc;
use h = hammer;

fn main() { 
  let parser = h::sequence([h::token("Hello, "), 
                            h::choice([h::token("there"), 
                                       h::token("world")]), 
                            h::token("!")]);

  println!("Try #1");
  println!("Result: {}", h::parse(parser, "Hello, world!").map_or("fail", |r| {r.pprint(); "success!"}));
  
  println!("Try #2");
  println!("Result: {}", h::parse(parser, "Hello, there!").map_or("fail", |r| {r.pprint(); "success!"}));
  
  println!("Try #3");
  println!("Result: {}", h::parse(parser, "What is happening?").map_or("fail", |r| {r.pprint(); "success!"}));
}


mod hammer;
mod hammerll;

