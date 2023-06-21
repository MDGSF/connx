use crate::minidds::common::error::Result;
use crate::minidds::dds::infrastructure::qos_policy::QosPolicies;

// OMG-DDS-v1.4-formal-15-04-10.pdf (2.2.2.1.1)
pub trait Entity {
    fn set_qos(&mut self, qos_list: &QosPolicies) -> Result<()>;
    fn get_qos(&self) -> QosPolicies;
    fn set_listener(&mut self) -> Result<()>;
    fn get_listener(&self);
    fn get_statuscondition(&self);
    fn get_status_changes(&self);
    fn enable(&mut self) -> Result<()>;
    fn get_instance_handle(&self);
}
