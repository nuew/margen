use hlua::{Lua, LuaError, LuaFunction};
use std::{error, fmt};
use std::io::prelude::*;

/// Lua enclosure
pub struct Generator<'b>(Lua<'b>);

impl<'b> Generator<'b> {
  pub fn ready<R: Read>(rdr: R) -> Result<Generator<'b>, GeneratorError> {
    info!("Creating lua context...");
    let mut lua = Lua::new();
    debug!("Loading libraries...");
    lua.openlibs();
    info!("Executing input script...");
    lua.execute_from_reader(rdr)?;
    Ok(Generator(lua))
  }

  pub fn get_boolean(&mut self, name: &str) -> Option<bool> {
    self.0.get(name)
  }

  pub fn get_float_32(&mut self, name: &str) -> Option<f32> {
    self.0.get(name)
  }

  pub fn get_integer_32(&mut self, name: &str) -> Option<i32> {
    self.0.get(name)
  }

  pub fn get_unsigned_32(&mut self, name: &str) -> Option<u32> {
    self.0.get(name)
  }

  pub fn get_unsigned_16(&mut self, name: &str) -> Option<u16> {
    self.0.get(name)
  }

  pub fn get_string(&mut self, name: &str) -> Option<String> {
    self.0.get(name)
  }

  pub fn get_color_value(&mut self, name: &str, x: u32, y: u32) -> Result<u8, GeneratorError> {
    self.0.set("x", x);
    self.0.set("y", y);

    let mut func: LuaFunction<_> = match self.0.get(name) {
      Some(v) => v,
      None => return Err(GeneratorError::NotFound(String::from(name))),
    };

    Ok(func.call()?)
  }
}

pub enum GeneratorError {
  LuaError(LuaError),
  NotFound(String),
}

impl fmt::Display for GeneratorError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      GeneratorError::LuaError(ref err) => {
        match *err {
          LuaError::SyntaxError(ref string) => string.fmt(f),
          LuaError::ExecutionError(ref string) => string.fmt(f),
          LuaError::ReadError(ref err) => err.fmt(f),
          LuaError::WrongType => write!(f, "The wrong type of data was received from lua"),
        }
      }
      GeneratorError::NotFound(ref string) => write!(f, "No variable or function for '{}'", string),
    }
  }
}

impl fmt::Debug for GeneratorError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    fmt::Display::fmt(self, f)
  }
}

impl error::Error for GeneratorError {
  fn description(&self) -> &str {
    match *self {
      GeneratorError::LuaError(ref err) => {
        match *err {
          LuaError::SyntaxError(ref string) => string.as_ref(),
          LuaError::ExecutionError(ref string) => string.as_ref(),
          LuaError::ReadError(ref err) => err.description(),
          LuaError::WrongType => "The wrong type of data was received from lua",
        }
      }
      GeneratorError::NotFound(_) => "No such variable or function",
    }
  }

  fn cause(&self) -> Option<&error::Error> {
    if let GeneratorError::LuaError(LuaError::ReadError(ref err)) = *self {
      Some(err)
    } else {
      None
    }
  }
}

impl From<LuaError> for GeneratorError {
  fn from(err: LuaError) -> GeneratorError {
    GeneratorError::LuaError(err)
  }
}
