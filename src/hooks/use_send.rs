use dioxus::prelude::*;

#[derive(Default, PartialEq, Debug, Clone)]
pub struct SendData {
    pub method: String,
    pub id_type: String,
    pub id_number: String,
    pub bank: String,
    pub account_type: String,
    pub account_number: String,
    pub currency: String,
    pub amount: u64,
    pub from: String,
    pub created_at: String,
    pub payment_at: String,
    pub payment_via: String,
    pub memo: String,
    pub notes: String,
}

pub fn use_send() -> UseSendState {
    let data = consume_context::<Signal<SendData>>();

    use_hook(|| UseSendState {
        inner: UseSendData { data },
    })
}
#[derive(Clone, Copy)]
pub struct UseSendState {
    inner: UseSendData,
}
#[derive(Clone, Copy, Default, Debug)]
pub struct UseSendData {
    data: Signal<SendData>,
}
impl UseSendState {
    pub fn get(&self) -> UseSendData {
        self.inner.clone()
    }
    pub fn get_data(&self) -> SendData {
        self.inner.data.read().clone()
    }
    pub fn set_data(&mut self, data: SendData) {
        let mut inner = self.inner.data.write();
        *inner = data;
    }
    pub fn data_mut(&mut self) -> Signal<SendData> {
        self.inner.data.clone()
    }
    pub fn default(&mut self) {
        self.inner = UseSendData::default();
    }
}
