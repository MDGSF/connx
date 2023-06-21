pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    Unsupported,
    BadParameter,
    PreconditionNotMet,
    OutOfResources,
    NotEnabled,
    ImmutablePolicy,
    InconsistentPolicy,
    AlreadyDeleted,
    Timeout,
    NoData,
    IllegalOperation,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsupported => write!(f, "dds: Unsupported (2)"),
            Self::BadParameter => write!(f, "dds: BadParameter (3)"),
            Self::PreconditionNotMet => write!(f, "dds: PreconditionNotMet (4)"),
            Self::OutOfResources => write!(f, "dds: OutOfResources (5)"),
            Self::NotEnabled => write!(f, "dds: NotEnabled (6)"),
            Self::ImmutablePolicy => write!(f, "dds: ImmutablePolicy (7)"),
            Self::InconsistentPolicy => write!(f, "dds: InconsistentPolicy (8)"),
            Self::AlreadyDeleted => write!(f, "dds: AlreadyDeleted (9)"),
            Self::Timeout => write!(f, "dds: Timeout (10)"),
            Self::NoData => write!(f, "dds: NoData (11)"),
            Self::IllegalOperation => write!(f, "dds: IllegalOperation (12)"),
        }
    }
}

impl std::error::Error for Error {}
