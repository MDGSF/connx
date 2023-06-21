#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ReturnCode {
    RetCodeOK = 0,
    RetCodeError = 1,
    RetCodeUnsupported = 2,
    RetCodeBadParameter = 3,
    RetCodePreconditionNotMet = 4,
    RetCodeOutOfResources = 5,
    RetCodeNotEnabled = 6,
    RetCodeImmutablePolicy = 7,
    RetCodeInconsistentPolicy = 8,
    RetCodeAlreadyDeleted = 9,
    RetCodeTimeout = 10,
    RetCodeNoData = 11,
    RetCodeIllegalOperation = 12,
}
