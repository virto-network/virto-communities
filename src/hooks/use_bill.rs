use dioxus::prelude::*;

#[derive(Default, PartialEq, Debug, Clone)]
pub struct BillData {
    pub method: String,
    pub id_type: String,
    pub id_number: String,
    pub invoice_number: String,
    pub bank: String,
    pub account_type: String,
    pub account_number: String,
    pub email_recipient: String,
    pub currency: String,
    pub amount: u64,
    pub from: String,
    pub invoice_start_at: String,
    pub invoice_end_at: String,
    pub payment_at: String,
    pub payment_via: String,
    pub memo: String,
    pub notes: String,
}

pub fn use_bill() -> UseBillState {
    let data = consume_context::<Signal<BillData>>();

    use_hook(|| UseBillState {
        inner: UseBillData { data },
    })
}
#[derive(Clone, Copy)]
pub struct UseBillState {
    inner: UseBillData,
}
#[derive(Clone, Copy, Default, Debug)]
pub struct UseBillData {
    data: Signal<BillData>,
}
impl UseBillState {
    pub fn get(&self) -> UseBillData {
        self.inner.clone()
    }
    pub fn get_data(&self) -> BillData {
        self.inner.data.read().clone()
    }
    pub fn set_data(&mut self, data: BillData) {
        let mut inner = self.inner.data.write();
        *inner = data;
    }
    pub fn data_mut(&mut self) -> Signal<BillData> {
        self.inner.data.clone()
    }
    pub fn default(&mut self) {
        self.inner = UseBillData::default();
    }
}
