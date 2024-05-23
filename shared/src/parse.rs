#[derive(Debug)]
pub enum Error {
    /// Error for environment variable not in the set of choices
    InvalidEnvVar,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid environment variable")
    }
}

/// Choices struct to constrain the value of an environment variable to a set of choices
pub struct Choices<T>(Vec<T>);

impl<T: std::cmp::PartialEq> Choices<T> {
    /// parse the value of the environment variable
    /// throws an error if the value is not in the set of choices
    /// returns the value if it is in the set of choices
    /// # Arguments
    /// * `value` - the value of the environment variable
    /// # Returns
    /// * `Result<String, Error>` - the value of the environment variable if it is in the set of choices
    /// * `Error` - an error if the value is not in the set of choices
    /// # Example
    /// ```
    /// use shared::parse::choice;
    ///
    /// let choices = choice(vec!["a", "b", "c"]);
    /// let value = choices.parse("a".to_string()).unwrap();
    /// assert_eq!(value, "a");
    ///
    /// let choices = choice(vec!["a", "b", "c"]);
    /// let value = choices.parse("d".to_string());
    /// assert!(value.is_err());
    /// ```
    pub fn parse<P>(&self, value: P) -> Result<T, Error>
    where
        P: Into<T>,
    {
        let value_converted: T = value.try_into().map_err(|_| Error::InvalidEnvVar)?;
        if !self.0.contains(&value_converted) {
            return Err(Error::InvalidEnvVar);
        }
        Ok(value_converted)
    }
}

/// use this function in your config struct when you want to constrain the value of an environment variable to a set of choices
/// chain this function with the parse method of the Choices struct to parse the value of the environment variable
/// # Arguments
/// * `choices` - a vector of strings that represent the set of choices
/// # Returns
/// * `Choices` - a Choices struct that can be used to parse the value of the environment variable
/// # Example
/// ```
/// use shared::parse::choice;
/// use std::env;
///
/// struct Config {
///    pub auth_driver: String,
/// }
///
/// impl Config {
///    pub fn new() -> Config {
///       dotenv::dotenv().ok();
///       Config {
///         auth_driver:  choice(vec!["session", "jwt"])
///    .parse(env::var("AUTH_DRIVER").expect("AUTH_DRIVER must be set"))
///    .expect("AUTH_DRIVER must be in choices"),
///      }
///   }
/// }
/// ```
pub fn choice(choices: Vec<&str>) -> Choices<String> {
    Choices(choices.iter().map(|s| s.to_string()).collect())
}

/// use this function in your config struct when you want to constrain the value of an environment variable to a boolean
/// chain this function with the parse method of the Choices struct to parse the value of the environment variable
/// # Returns
/// * `Choices` - a Choices struct that can be used to parse the value of the environment variable
/// # Example
/// ```
/// use shared::parse::boolean;
/// use std::env;
///
/// struct Config {
///   pub development: bool,
/// }
///
/// impl Config {
///     pub fn new() -> Config {
///         dotenv::dotenv().ok();
///         Config {
///             development: boolean()
///                 .parse::<bool>(
///                     env::var("DEVELOPMENT")
///                     .expect("DEVELOPMENT must be set")
///                     .parse()
///                     .unwrap(),
///                 )
///                 .expect("DEVELOPMENT must be a boolean"),
///         }
///     }
/// }
/// ```
pub fn boolean() -> Choices<bool> {
    Choices(vec![true, false])
}
