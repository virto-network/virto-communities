use dioxus::prelude::*;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IsTimestampHandled(pub bool);
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimestampValue(pub u64);
pub fn use_timestamp() -> UseTimestampState {
    let timestamp = consume_context::<Signal<TimestampValue>>();
    use_hook(|| UseTimestampState { timestamp })
}
#[derive(Clone, Copy)]
pub struct UseTimestampState {
    timestamp: Signal<TimestampValue>,
}
impl UseTimestampState {
    pub fn get(&self) -> TimestampValue {
        self.timestamp.read().clone()
    }
    pub fn set(&mut self, timestamp: TimestampValue) {
        let mut inner = self.timestamp.write();
        *inner = timestamp;
    }
}
