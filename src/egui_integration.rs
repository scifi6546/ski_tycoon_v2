use super::prelude::Event;
use egui::RawInput;
/// Struct used to get state
pub struct EguiRawInputAdaptor {}
impl EguiRawInputAdaptor {
    pub fn process_events(&self, events: Vec<Event>) -> RawInput {
        todo!()
    }
}
