use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegistrationStep {
    Entry,
    Email,
    EmailSent,
    Password,
    Profile,
    Success,
}

impl fmt::Display for RegistrationStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Entry => "entry",
            Self::Email => "email",
            Self::EmailSent => "email-sent",
            Self::Password => "password",
            Self::Profile => "profile",
            Self::Success => "success",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for RegistrationStep {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "entry" => Ok(Self::Entry),
            "email" => Ok(Self::Email),
            "email-sent" => Ok(Self::EmailSent),
            "password" => Ok(Self::Password),
            "profile" => Ok(Self::Profile),
            "success" => Ok(Self::Success),
            _ => Err(()),
        }
    }
}
