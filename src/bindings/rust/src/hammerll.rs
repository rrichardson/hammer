/* automatically generated by rust-bindgen */
/* ~/ThirdParty/rust-bindgen/bindgen -l hammer -match hammer.h -o src/hammerll.rs /usr/local/include/hammer/hammer.h -I
 * /usr/lib/gcc/x86_64-linux-gnu/4.8/include/ */
/* modified by Rick Richardson to get it to compile, 
 * please don't overwrite with bindgen unless you know what you're doing */

extern crate libc;
use libc::{ size_t, uint8_t, uint64_t, int64_t, FILE };

pub type HArena = libc::c_void;

pub enum Struct_HParseState_ { }
pub type HParseState = Struct_HParseState_;
pub type Enum_HParserBackend_ = ::libc::c_uint;
pub static PB_MIN: ::libc::c_uint = 0;
pub static PB_PACKRAT: ::libc::c_uint = 0;
pub static PB_REGULAR: ::libc::c_uint = 1;
pub static PB_LLk: ::libc::c_uint = 2;
pub static PB_LALR: ::libc::c_uint = 3;
pub static PB_GLR: ::libc::c_uint = 4;
pub static PB_MAX: ::libc::c_uint = 4;
pub type HParserBackend = Enum_HParserBackend_;
pub type Enum_HTokenType_ = ::libc::c_uint;
pub static TT_NONE: ::libc::c_uint = 1;
pub static TT_BYTES: ::libc::c_uint = 2;
pub static TT_SINT: ::libc::c_uint = 4;
pub static TT_UINT: ::libc::c_uint = 8;
pub static TT_SEQUENCE: ::libc::c_uint = 16;
pub static TT_RESERVED_1: ::libc::c_uint = 17;
pub static TT_ERR: ::libc::c_uint = 32;
pub static TT_USER: ::libc::c_uint = 64;
pub static TT_MAX: ::libc::c_uint = 65;
pub type HTokenType = Enum_HTokenType_;
#[repr(C)]
pub struct Struct_HCountedArray_ {
    pub capacity: size_t,
    pub used: size_t,
    pub arena: *mut HArena,
    pub elements: *mut *mut Struct_HParsedToken_,
}
pub type HCountedArray = Struct_HCountedArray_;
#[repr(C)]
pub struct Struct_HBytes_ {
    pub token: *const uint8_t,
    pub len: size_t,
}
pub type HBytes = Struct_HBytes_;
#[repr(C)]
pub struct HTokenData {
    pub data: [u64, ..2u],
}
#[repr(C)]
pub struct Struct_HParsedToken_ {
    pub token_type: HTokenType,
    pub data: HTokenData,
    pub index: size_t,
    pub bit_offset: ::libc::c_char,
}
impl HTokenData {
    pub fn bytes(&mut self) -> *mut HBytes {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn sint(&mut self) -> *mut int64_t {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn uint(&mut self) -> *mut uint64_t {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn dbl(&mut self) -> *mut ::libc::c_double {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn flt(&mut self) -> *mut ::libc::c_float {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn seq(&mut self) -> *mut *mut HCountedArray {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn user(&mut self) -> *mut *mut ::libc::c_void {
        unsafe { ::std::mem::transmute(self) }
    }
}
pub type HParsedToken = Struct_HParsedToken_;
#[repr(C)]
pub struct Struct_HParseResult_ {
    pub ast: *const HParsedToken,
    pub bit_length: int64_t,
    pub arena: *mut HArena,
}
pub type HParseResult = Struct_HParseResult_;
pub enum Struct_HBitWriter_ { }
pub type HBitWriter = Struct_HBitWriter_;
pub type HAction =
    ::std::option::Option<extern "C" fn
                              (arg1: *const HParseResult,
                               arg2: *mut ::libc::c_void)
                              -> *mut HParsedToken>;
pub type HPredicate =
    ::std::option::Option<extern "C" fn
                              (arg1: *mut HParseResult,
                               arg2: *mut ::libc::c_void) -> ::libc::c_int>;
pub enum Struct_HCFChoice_ { }
pub type HCFChoice = Struct_HCFChoice_;
pub enum Struct_HRVMProg_ { }
pub type HRVMProg = Struct_HRVMProg_;
pub enum Struct_HParserVtable_ { }
pub type HParserVtable = Struct_HParserVtable_;
#[repr(C)]
pub struct Struct_HParser_ {
    pub vtable: *const HParserVtable,
    pub backend: HParserBackend,
    pub backend_data: *mut ::libc::c_void,
    pub env: *mut ::libc::c_void,
    pub desugared: *mut HCFChoice,
}
pub type HParser = Struct_HParser_;
#[repr(C)]
pub struct Struct_HParserTestcase_ {
    pub input: *mut ::libc::c_uchar,
    pub length: size_t,
    pub output_unambiguous: *mut ::libc::c_char,
}
pub type HParserTestcase = Struct_HParserTestcase_;
#[repr(C)]
pub struct Struct_HCaseResult_ {
    pub success: ::libc::c_int,
    pub length: size_t,
}
#[repr(C)]
pub struct HResultTiming {
    pub data: [u64, ..1u],
}
impl HResultTiming {
    pub fn actual_results(&mut self) -> *mut *const ::libc::c_char {
        unsafe { ::std::mem::transmute(self) }
    }
    pub fn parse_time(&mut self) -> *mut size_t {
        unsafe { ::std::mem::transmute(self) }
    }
}
pub type HCaseResult = Struct_HCaseResult_;
#[repr(C)]
pub struct Struct_HBackendResults_ {
    pub backend: HParserBackend,
    pub compile_success: ::libc::c_int,
    pub n_testcases: size_t,
    pub failed_testcases: size_t,
    pub cases: *mut HCaseResult,
}
pub type HBackendResults = Struct_HBackendResults_;
#[repr(C)]
pub struct Struct_HBenchmarkResults_ {
    pub len: size_t,
    pub results: *mut HBackendResults,
}

pub type HAllocator = libc::c_void;

pub type HBenchmarkResults = Struct_HBenchmarkResults_;
#[link(name = "hammer")]
extern "C" {
    pub fn h_parse(parser: *const HParser, input: *const uint8_t,
                   length: size_t) -> *mut HParseResult;
    pub fn h_parse__m(mm__: *mut HAllocator, parser: *const HParser,
                      input: *const uint8_t, length: size_t) ->
     *mut HParseResult;
    pub fn h_token(str: *const uint8_t, len: size_t) -> *mut HParser;
    pub fn h_token__m(mm__: *mut HAllocator, str: *const uint8_t, len: size_t)
     -> *mut HParser;
    pub fn h_ch(c: uint8_t) -> *mut HParser;
    pub fn h_ch__m(mm__: *mut HAllocator, c: uint8_t) -> *mut HParser;
    pub fn h_ch_range(lower: uint8_t, upper: uint8_t) -> *mut HParser;
    pub fn h_ch_range__m(mm__: *mut HAllocator, lower: uint8_t,
                         upper: uint8_t) -> *mut HParser;
    pub fn h_int_range(p: *const HParser, lower: int64_t, upper: int64_t) ->
     *mut HParser;
    pub fn h_int_range__m(mm__: *mut HAllocator, p: *const HParser,
                          lower: int64_t, upper: int64_t) -> *mut HParser;
    pub fn h_bits(len: size_t, sign: ::libc::c_int) -> *mut HParser;
    pub fn h_bits__m(mm__: *mut HAllocator, len: size_t, sign: ::libc::c_int)
     -> *mut HParser;
    pub fn h_int64() -> *mut HParser;
    pub fn h_int64__m(mm__: *mut HAllocator) -> *mut HParser;
    pub fn h_int32() -> *mut HParser;
    pub fn h_int32__m(mm__: *mut HAllocator) -> *mut HParser;
    pub fn h_int16() -> *mut HParser;
    pub fn h_int16__m(mm__: *mut HAllocator) -> *mut HParser;
    pub fn h_int8() -> *mut HParser;
    pub fn h_int8__m(mm__: *mut HAllocator) -> *mut HParser;
    pub fn h_uint64() -> *mut HParser;
    pub fn h_uint64__m(mm__: *mut HAllocator) -> *mut HParser;
    pub fn h_uint32() -> *mut HParser;
    pub fn h_uint32__m(mm__: *mut HAllocator) -> *mut HParser;
    pub fn h_uint16() -> *mut HParser;
    pub fn h_uint16__m(mm__: *mut HAllocator) -> *mut HParser;
    pub fn h_uint8() -> *mut HParser;
    pub fn h_uint8__m(mm__: *mut HAllocator) -> *mut HParser;
    pub fn h_whitespace(p: *const HParser) -> *mut HParser;
    pub fn h_whitespace__m(mm__: *mut HAllocator, p: *const HParser) ->
     *mut HParser;
    pub fn h_left(p: *const HParser, q: *const HParser) -> *mut HParser;
    pub fn h_left__m(mm__: *mut HAllocator, p: *const HParser,
                     q: *const HParser) -> *mut HParser;
    pub fn h_right(p: *const HParser, q: *const HParser) -> *mut HParser;
    pub fn h_right__m(mm__: *mut HAllocator, p: *const HParser,
                      q: *const HParser) -> *mut HParser;
    pub fn h_middle(p: *const HParser, x: *const HParser, q: *const HParser)
     -> *mut HParser;
    pub fn h_middle__m(mm__: *mut HAllocator, p: *const HParser,
                       x: *const HParser, q: *const HParser) -> *mut HParser;
    pub fn h_action(p: *const HParser, a: HAction,
                    user_data: *mut ::libc::c_void) -> *mut HParser;
    pub fn h_action__m(mm__: *mut HAllocator, p: *const HParser, a: HAction,
                       user_data: *mut ::libc::c_void) -> *mut HParser;
    pub fn h_in(charset: *const uint8_t, length: size_t) -> *mut HParser;
    pub fn h_in__m(mm__: *mut HAllocator, charset: *const uint8_t,
                   length: size_t) -> *mut HParser;
    pub fn h_not_in(charset: *const uint8_t, length: size_t) -> *mut HParser;
    pub fn h_not_in__m(mm__: *mut HAllocator, charset: *const uint8_t,
                       length: size_t) -> *mut HParser;
    pub fn h_end_p() -> *mut HParser;
    pub fn h_end_p__m(mm__: *mut HAllocator) -> *mut HParser;
    pub fn h_nothing_p() -> *mut HParser;
    pub fn h_nothing_p__m(mm__: *mut HAllocator) -> *mut HParser;
    pub fn h_sequence(p: *mut HParser, ...) -> *mut HParser;
    pub fn h_sequence__m(mm__: *mut HAllocator, p: *mut HParser, ...) ->
     *mut HParser;
    pub fn h_sequence__mv(mm__: *mut HAllocator, p: *mut HParser, ...)
     -> *mut HParser;
    pub fn h_sequence__v(p: *mut HParser, ...) -> *mut HParser;
    pub fn h_sequence__a(args: *mut *mut ::libc::c_void) -> *mut HParser;
    pub fn h_sequence__ma(mm__: *mut HAllocator,
                          args: *mut *mut ::libc::c_void) -> *mut HParser;
    pub fn h_choice(p: *mut HParser, ...) -> *mut HParser;
    pub fn h_choice__m(mm__: *mut HAllocator, p: *mut HParser, ...) ->
     *mut HParser;
    pub fn h_choice__mv(mm__: *mut HAllocator, p: *mut HParser, ...)
     -> *mut HParser;
    pub fn h_choice__v(p: *mut HParser, ...) -> *mut HParser;
    pub fn h_choice__a(args: *mut *mut ::libc::c_void) -> *mut HParser;
    pub fn h_choice__ma(mm__: *mut HAllocator, args: *mut *mut ::libc::c_void)
     -> *mut HParser;
    pub fn h_butnot(p1: *const HParser, p2: *const HParser) -> *mut HParser;
    pub fn h_butnot__m(mm__: *mut HAllocator, p1: *const HParser,
                       p2: *const HParser) -> *mut HParser;
    pub fn h_difference(p1: *const HParser, p2: *const HParser) ->
     *mut HParser;
    pub fn h_difference__m(mm__: *mut HAllocator, p1: *const HParser,
                           p2: *const HParser) -> *mut HParser;
    pub fn h_xor(p1: *const HParser, p2: *const HParser) -> *mut HParser;
    pub fn h_xor__m(mm__: *mut HAllocator, p1: *const HParser,
                    p2: *const HParser) -> *mut HParser;
    pub fn h_many(p: *const HParser) -> *mut HParser;
    pub fn h_many__m(mm__: *mut HAllocator, p: *const HParser) ->
     *mut HParser;
    pub fn h_many1(p: *const HParser) -> *mut HParser;
    pub fn h_many1__m(mm__: *mut HAllocator, p: *const HParser) ->
     *mut HParser;
    pub fn h_repeat_n(p: *const HParser, n: size_t) -> *mut HParser;
    pub fn h_repeat_n__m(mm__: *mut HAllocator, p: *const HParser, n: size_t)
     -> *mut HParser;
    pub fn h_optional(p: *const HParser) -> *mut HParser;
    pub fn h_optional__m(mm__: *mut HAllocator, p: *const HParser) ->
     *mut HParser;
    pub fn h_ignore(p: *const HParser) -> *mut HParser;
    pub fn h_ignore__m(mm__: *mut HAllocator, p: *const HParser) ->
     *mut HParser;
    pub fn h_sepBy(p: *const HParser, sep: *const HParser) -> *mut HParser;
    pub fn h_sepBy__m(mm__: *mut HAllocator, p: *const HParser,
                      sep: *const HParser) -> *mut HParser;
    pub fn h_sepBy1(p: *const HParser, sep: *const HParser) -> *mut HParser;
    pub fn h_sepBy1__m(mm__: *mut HAllocator, p: *const HParser,
                       sep: *const HParser) -> *mut HParser;
    pub fn h_epsilon_p() -> *mut HParser;
    pub fn h_epsilon_p__m(mm__: *mut HAllocator) -> *mut HParser;
    pub fn h_length_value(length: *const HParser, value: *const HParser) ->
     *mut HParser;
    pub fn h_length_value__m(mm__: *mut HAllocator, length: *const HParser,
                             value: *const HParser) -> *mut HParser;
    pub fn h_attr_bool(p: *const HParser, pred: HPredicate,
                       user_data: *mut ::libc::c_void) -> *mut HParser;
    pub fn h_attr_bool__m(mm__: *mut HAllocator, p: *const HParser,
                          pred: HPredicate, user_data: *mut ::libc::c_void) ->
     *mut HParser;
    pub fn h_and(p: *const HParser) -> *mut HParser;
    pub fn h_and__m(mm__: *mut HAllocator, p: *const HParser) -> *mut HParser;
    pub fn h_not(p: *const HParser) -> *mut HParser;
    pub fn h_not__m(mm__: *mut HAllocator, p: *const HParser) -> *mut HParser;
    pub fn h_indirect() -> *mut HParser;
    pub fn h_indirect__m(mm__: *mut HAllocator) -> *mut HParser;
    pub fn h_bind_indirect(indirect: *mut HParser, inner: *const HParser);
    pub fn h_bind_indirect__m(mm__: *mut HAllocator, indirect: *mut HParser,
                              inner: *const HParser);
    pub fn h_with_endianness(endianness: ::libc::c_char, p: *const HParser) ->
     *mut HParser;
    pub fn h_with_endianness__m(mm__: *mut HAllocator,
                                endianness: ::libc::c_char, p: *const HParser)
     -> *mut HParser;
    pub fn h_parse_result_free(result: *mut HParseResult);
    pub fn h_parse_result_free__m(mm__: *mut HAllocator,
                                  result: *mut HParseResult);
    pub fn h_write_result_unamb(tok: *const HParsedToken) ->
     *mut ::libc::c_char;
    pub fn h_pprint(stream: *mut FILE, tok: *const HParsedToken,
                    indent: ::libc::c_int, delta: ::libc::c_int);
    pub fn h_compile(parser: *mut HParser, backend: HParserBackend,
                     params: *const ::libc::c_void) -> ::libc::c_int;
    pub fn h_compile__m(mm__: *mut HAllocator, parser: *mut HParser,
                        backend: HParserBackend,
                        params: *const ::libc::c_void) -> ::libc::c_int;
    pub fn h_bit_writer_new(mm__: *mut HAllocator) -> *mut HBitWriter;
    pub fn h_bit_writer_put(w: *mut HBitWriter, data: uint64_t,
                            nbits: size_t);
    pub fn h_bit_writer_get_buffer(w: *mut HBitWriter, len: *mut size_t) ->
     *const uint8_t;
    pub fn h_bit_writer_free(w: *mut HBitWriter);
    pub fn h_act_first(p: *const HParseResult, userdata: *mut ::libc::c_void)
     -> *mut HParsedToken;
    pub fn h_act_second(p: *const HParseResult, userdata: *mut ::libc::c_void)
     -> *mut HParsedToken;
    pub fn h_act_last(p: *const HParseResult, userdata: *mut ::libc::c_void)
     -> *mut HParsedToken;
    pub fn h_act_flatten(p: *const HParseResult,
                         userdata: *mut ::libc::c_void) -> *mut HParsedToken;
    pub fn h_act_ignore(p: *const HParseResult, userdata: *mut ::libc::c_void)
     -> *mut HParsedToken;
    pub fn h_benchmark(parser: *mut HParser, testcases: *mut HParserTestcase)
     -> *mut HBenchmarkResults;
    pub fn h_benchmark__m(mm__: *mut HAllocator, parser: *mut HParser,
                          testcases: *mut HParserTestcase) ->
     *mut HBenchmarkResults;
    pub fn h_benchmark_report(stream: *mut FILE,
                              results: *mut HBenchmarkResults);
    pub fn h_allocate_token_type(name: *const ::libc::c_char) -> HTokenType;
    pub fn h_get_token_type_number(name: *const ::libc::c_char) -> HTokenType;
    pub fn h_get_token_type_name(token_type: HTokenType) ->
     *const ::libc::c_char;
}
