// OMG-DDS-v1.4-formal-15-04-10.pdf (2.2.2.1.3)
pub trait QosPolicy {
    fn name(&self) -> String;
}

pub struct QosPolicies {}
