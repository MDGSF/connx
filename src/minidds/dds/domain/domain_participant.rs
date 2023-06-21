use std::sync::{Arc, Mutex};

pub type TDomainID = u16;

// OMG-DDS-v1.4-formal-15-04-10.pdf (2.2.2.2.1)
pub struct DomainParticipant {
    _inner: Arc<Mutex<DomainParticipantInner>>,
}

impl DomainParticipant {}

pub(crate) struct DomainParticipantInner {
    _domain_id: TDomainID,
}
