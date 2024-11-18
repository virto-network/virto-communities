use dioxus::prelude::*;

#[derive(Clone, Default, Debug)]
pub struct WithdrawForm {
    pub address: String,
    pub amount: String,
}

pub fn use_withdraw() -> UseWithdrawState {
    let withdraw = consume_context::<Signal<WithdrawForm>>();

    use_hook(|| UseWithdrawState {
        inner: UseWithdrawInner { withdraw },
    })
}

#[derive(Clone, Copy)]
pub struct UseWithdrawState {
    inner: UseWithdrawInner,
}

#[derive(Clone, Copy, Default)]
pub struct UseWithdrawInner {
    withdraw: Signal<WithdrawForm>,
}

impl UseWithdrawState {
    pub fn get(&self) -> UseWithdrawInner {
        self.inner
    }

    pub fn get_withdraw(&self) -> WithdrawForm {
        self.inner.withdraw.read().clone()
    }

    pub fn set_withdraw(&mut self, withdraw: WithdrawForm) {
        let mut inner = self.inner.withdraw.write();
        *inner = withdraw;
    }

    pub fn withdraw_mut(&mut self) -> Signal<WithdrawForm> {
        self.inner.withdraw
    }

    pub fn is_form_complete(&self) -> bool {
        let withdraw = self.inner.withdraw.read();

        !withdraw.address.is_empty() && !withdraw.amount.is_empty()
    }

    pub fn default(&mut self) {
        self.inner = UseWithdrawInner::default()
    }
}
