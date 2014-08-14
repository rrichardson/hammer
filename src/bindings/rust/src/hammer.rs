
extern crate libc;
use std::num::{ from_uint };
use std::ptr;
use std::mem;
use std::io::stdio;

use hammerll::*;

pub struct Parser {
  pub inner: *mut HParser
}

#[inline]
pub fn wrapParser(p: *mut HParser) -> Parser {
  Parser { inner: p }
}

pub struct ParseResult {
  inner: *mut HParseResult
}

impl ParseResult {
  pub fn pprint(&self) {
    unsafe {
      h_pprint(libc::fdopen(libc::STDOUT_FILENO, "w".to_c_str().as_ptr()), (*self.inner).ast, 0, 0)
    }
  }
}

pub fn parse(parser: Parser, input: &str) -> Option<ParseResult> {
  unsafe {
    let result = h_parse(&*parser.inner, input.as_ptr(), input.char_len() as u64);

    if (result == ptr::null::<HParseResult>() as *mut HParseResult) {
      None
    }
    else {
      Some(ParseResult { inner: result,  })
    }
  }
}


pub fn token(instr: &str) -> Parser {
  unsafe {
    wrapParser(h_token(instr.as_ptr(),  instr.char_len() as u64))
  }
}
pub fn ch(c: u8) -> Parser {
  unsafe {
    wrapParser(h_ch(c))
  }
}
pub fn ch_range(lower: u8, upper: u8) -> Parser {
  unsafe {
    wrapParser(h_ch_range(lower, upper))
  }
}
pub fn int_range(p: Parser, lower: i64, upper: i64) -> Parser {
  unsafe {
    wrapParser(h_int_range(&*p.inner, lower, upper))
  }
}
pub fn bits(len: u64, sign: i32) -> Parser {
  unsafe {
    wrapParser(h_bits(len, sign ))
  }
}
pub fn int64() -> Parser {
  unsafe {
    wrapParser(h_int64())
  }
}
pub fn int32() -> Parser {
  unsafe {
    wrapParser(h_int32())
  }
}
pub fn int16() ->  Parser {
  unsafe {
    wrapParser(h_int16())
  }
}
pub fn int8() ->  Parser {
  unsafe {
    wrapParser(h_int8())
  }
}
pub fn uint64() ->  Parser {
  unsafe {
    wrapParser(h_uint64())
  }
}
pub fn uint32() -> Parser {
  unsafe {
    wrapParser(h_uint32())
  }
}
pub fn uint16() -> Parser {
  unsafe {
    wrapParser(h_uint16())
  }
}
pub fn uint8() -> Parser {
  unsafe {
    wrapParser(h_uint8())
  }
}
pub fn whitespace(p: Parser) ->  Parser {
  unsafe {
    wrapParser(h_whitespace(&*p.inner))
  }
}
pub fn left(p: Parser, q: Parser) ->  Parser {
  unsafe {
    wrapParser(h_left(&*p.inner, &*q.inner))
  }
}
pub fn right(p: Parser, q: Parser) -> Parser {
  unsafe {
    wrapParser(h_right(&*p.inner, &*q.inner))
  }
}
pub fn middle(p: Parser, x: Parser, q: Parser) ->  Parser {
  unsafe {
    wrapParser(h_middle(&*p.inner, &*x.inner, &*q.inner))
  }
}

extern "C" fn act_cb<T>(arg1: *const HParseResult, arg2: *mut ::libc::c_void) -> *mut HParsedToken {
  unsafe {
    let cb  = *(arg2 as *mut fn (pr: Option<ParseResult>) -> Option<Box<T>>);
    let arg : Option<ParseResult> = 
              if arg1 == ptr::null::<HParseResult>() { 
                Some(ParseResult {inner: arg1 as *mut HParseResult}) } 
              else { None };
    let res : Option<Box<T>> = cb(arg);
    match res {
      Some(token) => {
        let retval : *mut ::libc::c_void = mem::transmute(token);
        h_make((*arg1).arena, TT_USER, retval)
      } 
      None => ptr::null::<HParsedToken>() as *mut HParsedToken 
    }
  }
}

pub fn action<T>(p: Parser, cb: fn (pr: Option<ParseResult>) -> Option<Box<T>>) ->  Parser {
  unsafe {
    let user_data = cb as *mut ::libc::c_void;
    wrapParser(h_action(&*p.inner, act_cb::<T>, user_data))
  }
}


pub fn inside(charset: &str) ->  Parser {
  unsafe {
    wrapParser(h_in(charset.as_ptr(), charset.char_len() as u64))
  }
}
pub fn not_in(charset: &str) ->  Parser {
  unsafe {
    wrapParser(h_not_in(charset.as_ptr(), charset.char_len() as u64))
  }
}
pub fn end() ->  Parser {
  unsafe {
    wrapParser(h_end_p())
  }
}
pub fn nothing() ->  Parser {
  unsafe {
    wrapParser(h_nothing_p())
  }
}
pub fn sequence(parsers: &[Parser]) -> Parser {
  unsafe {
    let mut ps : Vec<*mut HParser> = Vec::from_fn(parsers.len(), |i| parsers[i].inner);
    ps.push(ptr::null::<HParser>() as *mut HParser);
    wrapParser(h_sequence__a(ps.as_mut_ptr() as *mut *mut libc::c_void))
  }
}

pub fn choice(parsers: &[Parser]) -> Parser {
  unsafe {
    let mut ps : Vec<*mut HParser> = Vec::from_fn(parsers.len(), |i| parsers[i].inner);
    ps.push(ptr::null::<HParser>() as *mut HParser);
    wrapParser(h_choice__a(ps.as_mut_ptr() as *mut *mut libc::c_void))
  }
}

pub fn butnot(p1: Parser, p2: Parser) ->  Parser {
  unsafe {
    wrapParser(h_butnot(&*p1.inner, &*p2.inner))
  }
}
pub fn difference(p1: Parser, p2: Parser) ->  Parser {
  unsafe {
    wrapParser(h_difference(&*p1.inner, &*p2.inner))
  }
}
pub fn xor(p1: Parser, p2: Parser) ->  Parser {
  unsafe {
    wrapParser(h_xor(&*p1.inner, &*p2.inner))
  }
}
pub fn many(p: Parser) -> Parser {
  unsafe {
    wrapParser(h_many(&*p.inner))
  }
}
pub fn many1(p: Parser) -> Parser {
  unsafe {
    wrapParser(h_many1(&*p.inner))
  }
}
pub fn repeat(p: Parser, n: u64) -> Parser {
  unsafe {
    wrapParser(h_repeat_n(&*p.inner, n))
  }
}
pub fn optional(p: Parser) ->  Parser {
  unsafe {
    wrapParser(h_optional(&*p.inner))
  }
}
pub fn ignore(p: Parser) ->  Parser {
  unsafe {
    wrapParser(h_ignore(&*p.inner))
  }
}
pub fn sepBy(p: Parser, sep: Parser) ->  Parser {
  unsafe {
    wrapParser(h_sepBy(&*p.inner, &*sep.inner))
  }
}
pub fn sepBy1(p: Parser, sep: Parser) ->  Parser {
  unsafe {
    wrapParser(h_sepBy1(&*p.inner, &*sep.inner))
  }
}
pub fn epsilon() -> Parser {
  unsafe {
    wrapParser(h_epsilon_p())
  }
}
pub fn length_value(length: Parser, value: Parser) ->  Parser {
  unsafe {
    wrapParser(h_length_value(&*length.inner, &*value.inner))
  }
}
pub fn attr_bool(p: Parser, pred: HPredicate, user_data: *mut ::libc::c_void) ->  Parser {
  unsafe {
    wrapParser(h_attr_bool(&*p.inner, pred, user_data))
  }
}
pub fn and(p: Parser) -> Parser {
  unsafe {
    wrapParser(h_and(&*p.inner))
  }
}
pub fn not(p: Parser) -> Parser {
  unsafe {
    wrapParser(h_not(&*p.inner))
  }
}
pub fn indirect() -> Parser {
  unsafe {
    wrapParser(h_indirect())
  }
}
pub fn bind_indirect(indirect:Parser, inner: Parser) {
  unsafe {
    h_bind_indirect(indirect.inner, &*inner.inner)
  }
}
pub fn with_endianness(endianness: i8, p: Parser) -> Parser {
  unsafe {
    wrapParser(h_with_endianness(endianness, &*p.inner))
  }
}

