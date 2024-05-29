use dioxus::prelude::*;

use pjs::Account as PjsAccount;

pub type Account = PjsAccount;
pub type Accounts = Vec<Account>;

pub fn use_accounts() -> UseAccountsState {
    let accounts = consume_context::<Signal<Vec<Account>>>();
    let account = consume_context::<Signal<Option<Account>>>();

    use_hook(|| UseAccountsState {
        inner: accounts,
        account,
    })
}

#[derive(Clone, Copy)]
pub struct UseAccountsState {
    inner: Signal<Accounts>,
    account: Signal<Option<Account>>,
}

impl UseAccountsState {
    pub fn get(&self) -> Accounts {
        self.inner.read().clone()
    }

    pub fn get_one(&self, index: u8) -> Option<Account> {
        let accounts = self.inner.read();
        let account = accounts.get(index as usize);
        account.cloned()
    }

    pub fn set(&mut self, accounts: Accounts) {
        let mut inner = self.inner.write();
        *inner = accounts;
    }

    pub fn get_account(&self) -> Option<Account> {
        self.account.read().clone()
    }

    pub fn set_account(&mut self, account: Option<Account>) {
        let mut c = self.account.write();
        *c = account;
    }

    pub fn default(&mut self) {
        self.set(Accounts::default())
    }
}
