use sp_core::crypto::Ss58Codec;

use dioxus::{logger::tracing::{debug, warn}, prelude::*};
use dioxus_i18n::t;
use pjs::Account as PjsAccount;
use wasm_bindgen::prelude::*;

use crate::services::kreivo::{balances::account, communities::is_admin};

use super::{
    use_connect_wallet::{use_connect_wallet, PjsError},
    use_notification::use_notification,
    use_session::use_session,
};
pub type Account = PjsAccount;
pub type Accounts = Vec<Account>;
#[derive(Debug)]
pub struct AreAccountsInitialized(pub bool);
#[derive(Debug)]
pub struct IsDaoOwner(pub bool);
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = globalThis, js_name = initExecutor)]
    pub fn init_executor();
}
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = globalThis, js_name = setSigner)]
    pub fn set_signer(address: String);
}
pub fn use_accounts() -> UseAccountsState {
    
    let session = use_session();
    let mut notification = use_notification();
    let accounts = consume_context::<Signal<Vec<Account>>>();
    let mut account = consume_context::<Signal<Option<Account>>>();
    let mut is_dao_owner = consume_context::<Signal<Option<IsDaoOwner>>>();
    let mut are_accounts_initialized = consume_context::<Signal<AreAccountsInitialized>>();
    let pjs = use_context::<Signal<Option<pjs::PjsExtension>>>();

    use_coroutine(move |_: UnboundedReceiver<()>| async move {
        if session.is_logged() && pjs().is_none() {
            match use_connect_wallet().await {
                Err(PjsError::ConnectionFailed) => {
                    notification.handle_error(&t!("errors-wallet-connection_failed"))
                }
                Err(PjsError::AccountsNotFound) => {
                    notification.handle_error(&t!("errors-wallet-accounts_not_found"));
                }
                Ok(_) => {
                    if let Some(user_session) = session.get() {
                        let account_list = accounts();
                        let Some(selected_account) =
                            account_list.get(user_session.account_id as usize)
                        else {
                            return notification.handle_warning(
                                &t!("warnings-title"),
                                &t!("warnings-middleware-not_account"),
                            );
                        };

                        account.set(Some(selected_account.clone()));
                        set_signer(selected_account.address().clone());

                        let Ok(address) =
                            sp_core::sr25519::Public::from_ss58check(&selected_account.address())
                        else {
                            warn!("Not found public address");
                            return notification
                                .handle_error(&t!("errors-wallet-account_address"));
                        };

                        let Ok(is_owner) = is_admin(&address.0).await else {
                            warn!("Failed to get is admin");
                            return notification
                                .handle_error(&t!("errors-wallet-account_address"));
                        };

                        is_dao_owner.set(Some(IsDaoOwner(is_owner)));
                    }
                }
            };

            are_accounts_initialized.set(AreAccountsInitialized(true));
        }
    });

    use_hook(|| UseAccountsState {
        inner: accounts,
        account,
        is_dao_owner,
        are_accounts_initialized,
    })
}
#[derive(Clone, Copy)]
pub struct UseAccountsState {
    inner: Signal<Accounts>,
    account: Signal<Option<Account>>,
    is_dao_owner: Signal<Option<IsDaoOwner>>,
    are_accounts_initialized: Signal<AreAccountsInitialized>,
}
impl UseAccountsState {
    pub fn get(&self) -> Accounts {
        self.inner.read().clone()
    }
    pub fn get_one(&self, index: usize) -> Option<Account> {
        let accounts = self.inner.read();
        let account = accounts.get(index);
        account.cloned()
    }
    pub fn set(&mut self, accounts: Accounts) {
        let mut inner = self.inner.write();
        *inner = accounts;
    }
    pub fn get_account(&self) -> Option<Account> {
        self.account.read().clone()
    }
    pub fn set_account(&mut self, account_id: usize) -> Result<Account, String> {
        let selected_account = self
            .get_one(account_id)
            .ok_or("warning.middleware.not_account".to_string())?;

        let mut c = self.account.write();
        *c = Some(selected_account.clone());
        set_signer(selected_account.address().clone());

        Ok(selected_account.clone())
    }
    pub async fn get_balance(&self) -> Result<f64, String> {
        let pjs_account = self
            .get_account()
            .ok_or("errors.wallet.accounts_not_found")?;
        let account_address = pjs_account.address();
        let address = sp_core::sr25519::Public::from_ss58check(&account_address).map_err(|e| {
            warn!("Not found public address: {:?}", e);
            "errors.wallet.account_address".to_string()
        })?;
        let hex_address = hex::encode(address.0);
        let account = account(&format!("0x{}", hex_address))
            .await
            .map_err(|_| "errors.wallet.accounts_not_found".to_string())?;
        Ok(account.data.free as f64 / 10_f64.powf(12f64))
    }
    pub fn is_active_account_an_admin(&self) -> bool {
        debug!("is_dao_owner: {:?}", self.is_dao_owner.read());
        match &*self.is_dao_owner.read() {
            Some(is_dao_owner) => is_dao_owner.0,
            None => true,
        }
    }
    pub fn are_accounts_initialized(&self) -> bool {
        self.are_accounts_initialized.read().0
    }
    pub fn set_is_active_account_an_admin(&mut self, is_dao_owner: Option<IsDaoOwner>) {
        let mut c = self.is_dao_owner.write();
        *c = is_dao_owner;
    }
    pub fn default(&mut self) {
        self.set(Accounts::default())
    }
}
