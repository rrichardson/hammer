extern crate libc;
use std::num::{ from_uint };
use std::ptr;
use std::mem;
use std::io::stdio;
use h = hammerll;

static hello : &'static str = "Hello ";
static world : &'static str = "World";
static there : &'static str = "there";
static bang : &'static str = "!";

fn init_parser() -> *mut h::HParser {
  unsafe {
    let token1 = h::h_token(hello.as_ptr(), from_uint(hello.char_len()).expect("uint conversion failed"));
    let token2 = h::h_token(world.as_ptr(), from_uint(world.char_len()).expect("uint conversion failed"));
    let token3 = h::h_token(there.as_ptr(), from_uint(there.char_len()).expect("uint conversion failed"));
    let token4 = h::h_token(bang.as_ptr(), from_uint(bang.char_len()).expect("uint conversion failed"));
    h::h_sequence(token1, h::h_choice(token2, token3, token4, ptr::null::<h::HParser>()) , ptr::null::<h::HParser>())
  }
}

fn run_parser(parser: *mut h::HParser, input: &str) -> *mut h::HParseResult {
  unsafe {
    h::h_parse(&*parser, input.as_ptr(), from_uint(input.char_len()).expect("blarg"))
  }
}

fn main() { 
  let parser = init_parser();
  let result =  run_parser(parser, "Hello World!");
  unsafe {
    h::h_pprint(libc::fdopen(libc::STDOUT_FILENO, mem::transmute("w".as_ptr())), (*result).ast, 0, 0);
  }
}

mod hammerll;
