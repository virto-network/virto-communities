use dioxus::prelude::*;
#[derive(Default, PartialEq, Debug, Clone)]
pub struct InvoiceX {
    pub name: String,
    pub email: String,
    pub address: String,
    pub zip: String,
    pub country: String,
}
#[derive(Default, PartialEq, Debug, Clone)]
pub struct Tax {
    pub percent: u8,
    pub value: f64,
}
#[derive(Default, PartialEq, Debug, Clone)]
pub struct InvoiceDetails {
    pub id: String,
    pub tax: Tax,
    pub total: f64,
}
#[derive(Default, PartialEq, Debug, Clone)]
pub struct InvoiceItem {
    pub description: String,
    pub quantity: u64,
    pub amount: f64,
}
#[derive(Default, PartialEq, Debug, Clone)]
pub struct InvoiceTerms {
    pub created_at: String,
    pub payment_at: String,
    pub payment_via: String,
    pub memo: String,
}
#[derive(Default, PartialEq, Debug, Clone)]
pub struct InvoiceTransfer {
    pub name: String, // Bank name, chain name
    pub account: String,
    pub account_type: String,
    pub account_number: String,
    pub routing_number: String,
}
#[derive(Default, PartialEq, Debug, Clone)]
pub struct InvoiceItems {
    pub items: Vec<InvoiceItem>,
}
pub fn use_invoice() -> UseInvoiceState {
    // let from = use_signal::<InvoiceX>(|| InvoiceX::default());
    // let to = use_signal::<InvoiceX>(|| InvoiceX::default());
    // let details = use_signal::<InvoiceDetails>(|| InvoiceDetails::default());
    // let items = use_signal::<InvoiceItems>(|| InvoiceItems::default());
    // let terms = use_signal::<InvoiceTerms>(|| InvoiceTerms::default());
    // let transfer = use_signal::<InvoiceTransfer>(|| InvoiceTransfer::default());

    let from = use_signal::<InvoiceX>(|| InvoiceX {
        name: "Virto Network Solutions LLC".to_string(),
        email: "invoices@virto.com".to_string(),
        address: "333 Bush Street, Suite 1900".to_string(),
        zip: "San Francisco, CA 94104".to_string(),
        country: "United States of America".to_string(),
    });

    let to = use_signal::<InvoiceX>(|| InvoiceX {
        name: "TechVista Solutions".to_string(),
        email: "invoices@techvistasolutions.com".to_string(),
        address: "333 Bush Street, Suite 1900".to_string(),
        zip: "San Francisco, CA 94104".to_string(),
        country: "United States of America".to_string(),
    });

    let details = use_signal::<InvoiceDetails>(|| InvoiceDetails {
        id: "INV-1267".to_string(),
        tax: Tax {
            percent: 0,
            value: 0.0,
        },
        total: 1300.0,
    });

    let items = use_signal::<InvoiceItems>(|| InvoiceItems {
        items: vec![
            InvoiceItem {
                description: "AI chatbot/Enterprise".to_string(),
                quantity: 1,
                amount: 1000.0,
            },
            InvoiceItem {
                description: "AI chatbot/Enterprise".to_string(),
                quantity: 1,
                amount: 300.0,
            },
        ],
    });

    let terms = use_signal::<InvoiceTerms>(|| {
        InvoiceTerms {
        created_at: "07/29/24".to_string(),
        payment_at: "08/28/24".to_string(),
        payment_via: "ACH, Card, Wire, Pay byVirto".to_string(),
        memo: "Office ipsum you must be muted. Asserts yet silently today status. Items / window / gave focus i’m. Create sky shower level revision minimize. Scope caught horse spaces pivot pants own.".to_string(),
    }
    });

    let transfer = use_signal::<InvoiceTransfer>(|| InvoiceTransfer {
        name: "Bank of America".to_string(),
        account: "Tax Bureau Inc.".to_string(),
        account_type: "Business Checking".to_string(),
        account_number: "155456789".to_string(),
        routing_number: "9090101".to_string(),
    });

    use_hook(|| UseInvoiceState {
        inner: UseInvoiceInner {
            from,
            to,
            details,
            items,
            terms,
            transfer,
        },
    })
}
#[derive(Clone, Copy)]
pub struct UseInvoiceState {
    inner: UseInvoiceInner,
}
#[derive(Clone, Copy, Default)]
pub struct UseInvoiceInner {
    from: Signal<InvoiceX>,
    to: Signal<InvoiceX>,
    details: Signal<InvoiceDetails>,
    items: Signal<InvoiceItems>,
    terms: Signal<InvoiceTerms>,
    transfer: Signal<InvoiceTransfer>,
}
impl UseInvoiceState {
    pub fn get(&self) -> UseInvoiceInner {
        self.inner.clone()
    }
    pub fn get_from(&self) -> InvoiceX {
        self.inner.from.read().clone()
    }
    pub fn set_from(&mut self, from: InvoiceX) {
        let mut inner = self.inner.from.write();
        *inner = from;
    }
    pub fn from_mut(&mut self) -> Signal<InvoiceX> {
        self.inner.from.clone()
    }
    pub fn get_to(&self) -> InvoiceX {
        self.inner.to.read().clone()
    }
    pub fn set_to(&mut self, to: InvoiceX) {
        let mut inner = self.inner.to.write();
        *inner = to;
    }
    pub fn to_mut(&mut self) -> Signal<InvoiceX> {
        self.inner.to.clone()
    }
    pub fn get_details(&self) -> InvoiceDetails {
        self.inner.details.read().clone()
    }
    pub fn set_details(&mut self, details: InvoiceDetails) {
        let mut inner = self.inner.details.write();
        *inner = details;
    }
    pub fn details_mut(&mut self) -> Signal<InvoiceDetails> {
        self.inner.details.clone()
    }
    pub fn get_items(&self) -> InvoiceItems {
        self.inner.items.read().clone()
    }
    pub fn set_items(&mut self, items: InvoiceItems) {
        let mut inner = self.inner.items.write();
        *inner = items;
    }
    pub fn push_item(&mut self, item: InvoiceItem) {
        self.inner.items.with_mut(|i| i.items.push(item));
    }
    pub fn remove_item(&mut self, position: usize) {
        self.inner.items.with_mut(|i| i.items.remove(position));
    }
    pub fn update_item(&mut self, position: usize, item: InvoiceItem) {
        self.inner.items.with_mut(|i| i.items[position] = item);
    }
    pub fn get_terms(&self) -> InvoiceTerms {
        self.inner.terms.read().clone()
    }
    pub fn set_terms(&mut self, terms: InvoiceTerms) {
        let mut inner = self.inner.terms.write();
        *inner = terms;
    }
    pub fn terms_mut(&mut self) -> Signal<InvoiceTerms> {
        self.inner.terms.clone()
    }
    pub fn get_transfer(&self) -> InvoiceTransfer {
        self.inner.transfer.read().clone()
    }
    pub fn set_transfer(&mut self, transfer: InvoiceTransfer) {
        let mut inner = self.inner.transfer.write();
        *inner = transfer;
    }
    pub fn transfer_mut(&mut self) -> Signal<InvoiceTransfer> {
        self.inner.transfer.clone()
    }
    pub fn default(&mut self) {
        self.inner = UseInvoiceInner::default();
    }
}
