pub mod topic_description;

use crate::minidds::dds::domain::domain_participant::DomainParticipant;
use topic_description::TopicDescription;

pub struct Topic {}

impl TopicDescription for Topic {
    fn get_participant(&self) -> Option<DomainParticipant> {
        None
    }

    fn get_type_name(&self) -> String {
        String::from("")
    }
    fn get_name(&self) -> String {
        String::from("")
    }
}
