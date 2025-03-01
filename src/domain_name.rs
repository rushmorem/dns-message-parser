use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::hash::{Hash, Hasher};
use thiserror::Error;

pub const DOMAIN_NAME_MAX_RECURSION: usize = 16;
pub const DOMAIN_NAME_MAX_LABEL_LENGTH: usize = 64;
pub const DOMAIN_NAME_MAX_LENGTH: usize = 256;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum DomainNameError {
    #[error("Label is too big: {DOMAIN_NAME_MAX_LABEL_LENGTH} <= {0}")]
    LabelLength(usize),
    #[error("Domain name is too big: {DOMAIN_NAME_MAX_LENGTH} <= {0}")]
    DomainNameLength(usize),
    #[error("Label contains dot")]
    LabelDot,
}

#[derive(Debug, Clone, Eq)]
pub struct DomainName(pub(super) String);

impl DomainName {
    /// Append a label to the domain name.
    ///
    /// If the label cannot be appended then the domain name is not changed.
    /// The label cannot be appended if the label is too big **or** the label contains a dot
    /// character **or** the domain name would be too big.
    ///
    /// # Example
    /// ```
    /// # use dns_message_parser::DomainName;
    /// let mut domain_name = DomainName::default();
    /// // Prints "."
    /// println!("{}", domain_name);
    ///
    /// domain_name.append_label("example").unwrap();
    /// // Prints "example."
    /// println!("{}", domain_name);
    ///
    /// domain_name.append_label("org").unwrap();
    /// // Prints "example.org."
    /// println!("{}", domain_name);
    ///
    /// let result = domain_name.append_label(".");
    /// // Prints "true"
    /// println!("{}", result.is_err());
    /// // Prints "example.org."
    /// println!("{}", domain_name);
    /// ```
    pub fn append_label(&mut self, label: &str) -> Result<(), DomainNameError> {
        let label_length = label.len();
        if DOMAIN_NAME_MAX_LABEL_LENGTH <= label_length {
            return Err(DomainNameError::LabelLength(label_length));
        }

        let domain_name_length = self.0.len() + label_length;
        if DOMAIN_NAME_MAX_LENGTH <= domain_name_length {
            return Err(DomainNameError::DomainNameLength(domain_name_length));
        }

        if label.contains('.') {
            return Err(DomainNameError::LabelDot);
        }

        if &self.0 == "." {
            self.0.insert_str(0, label);
        } else {
            self.0.push_str(label);
            self.0.push('.');
        }
        Ok(())
    }
}

impl TryFrom<&str> for DomainName {
    type Error = DomainNameError;

    fn try_from(string: &str) -> Result<Self, DomainNameError> {
        let string_relativ = if let Some(string_relativ) = string.strip_suffix('.') {
            string_relativ
        } else {
            string
        };

        let mut domain_name = DomainName::default();
        for label in string_relativ.split('.') {
            domain_name.append_label(label)?;
        }
        Ok(domain_name)
    }
}

impl Default for DomainName {
    fn default() -> Self {
        DomainName(".".to_string())
    }
}

impl From<DomainName> for String {
    fn from(domain_name: DomainName) -> Self {
        domain_name.0
    }
}

impl AsRef<str> for DomainName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq<DomainName> for DomainName {
    fn eq(&self, other: &DomainName) -> bool {
        self.0.to_lowercase() == other.0.to_lowercase()
    }
}

impl PartialEq<&str> for DomainName {
    fn eq(&self, other: &&str) -> bool {
        self.0.to_lowercase() == other.to_lowercase()
    }
}

impl Display for DomainName {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl Hash for DomainName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_lowercase().hash(state);
    }
}
