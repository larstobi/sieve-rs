use std::error::Error;
use std::fmt;

use nom::{Needed,IResult};

use parser;

#[derive(Clone,PartialEq,Debug)]
pub enum CompileError<'a> {
  Incomplete(Needed),
  Unknown(IResult<&'a str,Vec<parser::Command>>),
}

impl<'a> Error for CompileError<'a> {
  fn description(&self) -> &str {
    match *self {
      CompileError::Incomplete(_) => "unexpected end-of-file",
      CompileError::Unknown(_)    => "unknown error",
    }
  }
}

impl<'a> fmt::Display for CompileError<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", match *self {
      CompileError::Incomplete(ref r) => format!("{} ({:?})", self.description(), r),
      CompileError::Unknown(ref r)    => format!("{} ({:?})", self.description(), r),
    }.to_string())
  }
}