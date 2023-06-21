// OMG-DDS-v1.4-formal-15-04-10.pdf (2.2.2.1.1)
pub trait Entity {
    fn set_qos();
    fn get_qos();
    fn set_listener();
    fn get_listener();
    fn get_statuscondition();
    fn get_status_changes();
    fn enable();
    fn get_instance_handle();
}
