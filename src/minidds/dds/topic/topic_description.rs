use crate::minidds::dds::domain::domain_participant::DomainParticipant;

// OMG-DDS-v1.4-formal-15-04-10.pdf (2.2.2.3.1)
pub trait TopicDescription {
    /// This operation returns the DomainParticipant to which the TopicDescription belongs.
    fn get_participant(&self) -> Option<DomainParticipant>;
    fn get_type_name(&self) -> String;
    fn get_name(&self) -> String;
}
