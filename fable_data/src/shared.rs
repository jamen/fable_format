// Re-exports

use std::io::{Read,Write,Seek};

use super::Error;

pub(crate) use nom::*;
pub(crate) use nom::branch::*;
pub(crate) use nom::bytes::complete::*;
pub(crate) use nom::character::complete::*;
pub(crate) use nom::combinator::*;
pub(crate) use nom::multi::*;
pub(crate) use nom::number::complete::*;
pub(crate) use nom::sequence::*;
pub(crate) use nom::error::ParseError;

// Encoding/decoding

pub trait Decode: Sized {
    type Error;
    fn decode<Source: Read + Seek>(source: &mut Source) -> Result<Self, Self::Error>;
}

pub trait Encode {
    type Error;
    fn encode<Target: Write + Seek>(&self, source: &mut Target) -> Result<(), Self::Error>;
}

// Generic parsers

pub fn decode_rle_string(input: &[u8]) -> IResult<&[u8], String, Error> {
    let (input, size) = le_u32(input)?;
    let (input, string) = take(size as usize)(input)?;
    let (_, string) = all_consuming(decode_bytes_as_utf8_string)(&string)?;
    Ok((input, string))
}

pub fn decode_null_terminated_string(input: &[u8]) -> IResult<&[u8], String, Error> {
    let (input, string) = take_till(|c| c == b'\0')(input)?;
    let (input, _nul) = tag(b"\0")(input)?;
    let (_, string) = all_consuming(decode_bytes_as_utf8_string)(&string)?;
    Ok((input, string))
}

pub fn decode_bytes_as_utf8_string(input: &[u8]) -> IResult<&[u8], String, Error> {
    match String::from_utf8(input.to_vec()) {
        Ok(string) => Ok((b"", string)),
        Err(_error) => Err(nom::Err::Error(Error::Utf8Error))
    }
}

// pub fn many_if<Input, Output, Parser, Cond, Error>(
//     parser: Parser,
//     cond: Cond
// ) -> impl Fn(I) -> IResult<I, Vec<O>, E> where
//     Input: Clone + PartialEq,
//     Output,
//     Parser: Fn(Input) -> IResult<Input, Output, Error>,
//     Cond: Fn(Output) -> bool,
//     Error: ParserError<Input>,
// {
//     move |i: Input| {
//     }
// }
