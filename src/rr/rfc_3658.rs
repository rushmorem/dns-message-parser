use super::Class;
use crate::DomainName;
use std::fmt::{Display, Formatter, Result as FmtResult};

try_from_enum_to_integer! {
    #[repr(u8)]
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    pub enum SSHFPAlgorithm {
        Reserved = 0,
        RSA = 1,
        DSS = 2,
    }
}

impl Display for SSHFPAlgorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            SSHFPAlgorithm::Reserved => write!(f, "Reserved"),
            SSHFPAlgorithm::RSA => write!(f, "RSA"),
            SSHFPAlgorithm::DSS => write!(f, "DSS"),
        }
    }
}

try_from_enum_to_integer! {
    #[repr(u8)]
    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    pub enum SSHFPType {
        Reserved = 0,
        Sha1 = 1,
    }
}

impl Display for SSHFPType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            SSHFPType::Reserved => write!(f, "Reserved"),
            SSHFPType::Sha1 => write!(f, "SHA-1"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SSHFP {
    pub domain_name: DomainName,
    pub ttl: u32,
    pub class: Class,
    pub algorithm: SSHFPAlgorithm,
    pub type_: SSHFPType,
    pub fp: Vec<u8>,
}

impl_to_type!(SSHFP);

impl Display for SSHFP {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "{} {} {} SSHFP {} {} {:x?}",
            self.domain_name, self.ttl, self.class, self.algorithm, self.type_, self.fp,
        )
    }
}
