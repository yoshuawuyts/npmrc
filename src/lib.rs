//! Read out npm's `.npmrc` file, and serialize it to a struct.
//!
//! ## Usage
//!
//! ```rust,ignore
//! extern crate npmrc;
//! let npmrc_values = npmrc::read().unwrap();
//! println!("{:?}", npmrc_values);
//! ```
#[macro_use(format_err)]
extern crate failure;
extern crate serde;
#[macro_use(Deserialize)]
extern crate serde_derive;
extern crate serde_ini;

use failure::Error;
use std::fs;
use std::str::FromStr;
use serde::{de, Deserialize, Deserializer};

// `serde_ini` only supports serializing to string types, so we have to define
// a custom deserializer.
fn de_from_str<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
  D: Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  bool::from_str(&s).map_err(de::Error::custom)
}

/// Npm's access levels.
#[derive(Debug, Deserialize)]
pub enum Access {
  /// Access is public.
  Public,

  /// Access is not public.
  Restricted,
}

/// Npm's log levels.
#[derive(Debug, Deserialize)]
pub enum LogLevel {
  /// No messages.
  #[serde(rename = "silent")]
  Silent,

  /// Log out error messages.
  #[serde(rename = "error")]
  Error,

  /// Log out warnings.
  #[serde(rename = "warn")]
  Warn,

  /// Log out notices.
  #[serde(rename = "notice")]
  Notice,

  /// Log out HTTP requests.
  #[serde(rename = "http")]
  Http,

  /// Log out timing information.
  #[serde(rename = "timing")]
  Timing,

  /// Log out a balanced amount of information.
  #[serde(rename = "info")]
  Info,

  /// Log out most things.
  #[serde(rename = "verbose")]
  Verbose,

  /// Log out everything.
  #[serde(rename = "silly")]
  Silly,
}

/// Representation of `.npmrc`.
#[derive(Debug, Deserialize)]
pub struct Npmrc {
  /// When publishing scoped packages, the access level defaults to `restricted`.
  /// If you want your scoped package to be publicly viewable (and installable)
  /// set `--access=public`. The only valid values for `access` are `public` and
  /// `restricted`. Unscoped packages always have an access level of `public`.
  /// [Read More.](https://docs.npmjs.com/misc/config#access)
  #[serde(default)]
  pub access: String,

  /// Set npm's log level.
  #[serde(default)]
  pub loglevel: String,

  /// Should npm echo out progress while installing packages?
  #[serde(default, deserialize_with = "de_from_str")]
  pub progress: bool,

  /// Should npm create a package-lock.json file?
  #[serde(rename = "package-lock")]
  #[serde(default, deserialize_with = "de_from_str")]
  pub package_lock: bool,

  /// Should npm modify package.json when installing?
  #[serde(default, deserialize_with = "de_from_str")]
  pub save: bool,

  /// The value `npm init` should use by default for the package author's name.
  #[serde(default, rename = "init-author-name")]
  pub init_author_name: String,

  /// The value `npm init` should use by default for the package author's email.
  #[serde(default, rename = "init-author-email")]
  pub init_author_email: String,
}

/// Read out `.npmrc` and return it.
pub fn read() -> Result<Npmrc, Error> {
  let npmrc_path = match std::env::home_dir() {
    None => return Err(format_err!("User's home directory not found")),
    Some(home_path) => home_path.join(".npmrc"),
  };

  let npmrc = fs::read_to_string(npmrc_path)?;

  let contents = serde_ini::from_str(&npmrc)?;
  Ok(contents)
}
