use dioxus::prelude::*;

use pjs::Account as PjsAccount;

pub type Account = PjsAccount;
pub type Accounts = Vec<Account>;

pub struct IsDaoOwner(pub bool);

pub fn use_accounts() -> UseAccountsState {
    let accounts = consume_context::<Signal<Vec<Account>>>();
    let account = consume_context::<Signal<Option<Account>>>();
    let is_dao_owner = consume_context::<Signal<IsDaoOwner>>();

    use_hook(|| UseAccountsState {
        inner: accounts,
        account,
        is_dao_owner,
    })
}

#[derive(Clone, Copy)]
pub struct UseAccountsState {
    inner: Signal<Accounts>,
    account: Signal<Option<Account>>,
    is_dao_owner: Signal<IsDaoOwner>,
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

    pub fn is_active_account_an_admin(&self) -> bool {
        self.is_dao_owner.read().0.clone()
    }

    pub fn set_is_active_account_an_admin(&mut self, is_dao_owner: IsDaoOwner) {
        let mut c = self.is_dao_owner.write();
        *c = is_dao_owner;
    }

    pub fn default(&mut self) {
        self.set(Accounts::default())
    }
}
