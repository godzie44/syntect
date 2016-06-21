//! Welcome to the syntect docs.
//! These are still a work in progress but a lot of the important things have
//! been documented already.
//!
//! Much more info about syntect is available on the [Github Page](https://github.com/trishume/syntect).
//!
//! May I suggest that you start by reading the `Readme.md` file in the main repo.
//! Once you're done with that you can look at the docs for `parsing::SyntaxSet`
//! and for the `easy` module.
//!
//! Almost everything in syntect is divided up into either the `parsing` module
//! for turning text into text annotated with scopes, and the `highlighting` module
//! for turning annotated text into styled/coloured text.
//!
//! Some docs have example code but a good place to look is the `syncat` example as well as the source code
//! for the `easy` module in `easy.rs` as that shows how to plug the various parts together for common use cases.
extern crate yaml_rust;
extern crate onig;
extern crate walkdir;
extern crate regex_syntax;
#[macro_use]
extern crate lazy_static;
extern crate plist;
extern crate bincode;
extern crate rustc_serialize;
#[macro_use]
extern crate bitflags;
extern crate flate2;
pub mod highlighting;
pub mod parsing;
pub mod util;
pub mod dumps;
pub mod easy;
pub mod html;

use std::io::Error as IoError;
use parsing::ParseSyntaxError;
use highlighting::{ParseThemeError, SettingsError};

/// Common error type used by syntax and theme loading
#[derive(Debug)]
pub enum LoadingError {
    /// error finding all the files in a directory
    WalkDir(walkdir::Error),
    /// error reading a file
    Io(IoError),
    /// a syntax file was invalid in some way
    ParseSyntax(ParseSyntaxError),
    /// a theme file was invalid in some way
    ParseTheme(ParseThemeError),
    /// a theme's Plist syntax was invalid in some way
    ReadSettings(SettingsError),
    /// A path given to a method was invalid.
    /// Possibly because it didn't reference a file or wasn't UTF-8.
    BadPath,
}

impl From<SettingsError> for LoadingError {
    fn from(error: SettingsError) -> LoadingError {
        LoadingError::ReadSettings(error)
    }
}

impl From<IoError> for LoadingError {
    fn from(error: IoError) -> LoadingError {
        LoadingError::Io(error)
    }
}

impl From<ParseThemeError> for LoadingError {
    fn from(error: ParseThemeError) -> LoadingError {
        LoadingError::ParseTheme(error)
    }
}

impl From<ParseSyntaxError> for LoadingError {
    fn from(error: ParseSyntaxError) -> LoadingError {
        LoadingError::ParseSyntax(error)
    }
}
