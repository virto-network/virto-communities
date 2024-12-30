use sp_core::crypto::Ss58Codec;

use dioxus::{logger::tracing::warn, prelude::*};

#[derive(Clone, Debug)]
pub enum DepositTo {
    Address(String),
    Community(u16),
}

#[derive(Clone, Default, Debug)]
pub struct DepositForm {
    pub dest: DepositTo,
    pub amount: String,
}

pub enum DepositError {
    MalformedAddress,
    InvalidAmount,
}

impl DepositForm {
    pub fn is_valid(&self) -> bool {
        let has_info = match &self.dest {
            // Check if is a valid address
            DepositTo::Address(addrs) => !addrs.is_empty(),
            DepositTo::Community(_) => true,
        };

        has_info && !self.amount.is_empty()
    }

    pub fn address(&self) -> String {
        match &self.dest {
            DepositTo::Address(addrs) => addrs.to_string(),
            _ => String::new(),
        }
    }

    pub fn to_deposit(&self) -> Result<(String, u64, bool), DepositError> {
        let amount = self.amount.parse::<f64>().map_err(|_| {
            warn!("Malformed amount");
            DepositError::InvalidAmount
        })?;
        let amount = (amount * 1_000_000_000_000.0) as u64;
        match &self.dest {
            DepositTo::Address(addrs) => {
                let address = sp_core::sr25519::Public::from_ss58check(addrs)
                    .map_err(|_| DepositError::MalformedAddress)?;
                let hex_address = format!("0x{}", hex::encode(address.0));
                Ok((hex_address, amount, false))
            }
            DepositTo::Community(id) => Ok((id.to_string(), amount, true)),
        }
    }
}

impl Default for DepositTo {
    fn default() -> Self {
        Self::Address(String::new())
    }
}

pub fn use_deposit() -> UseDepositState {
    let deposit = consume_context::<Signal<DepositForm>>();

    use_hook(|| UseDepositState {
        inner: UseDepositInner { deposit },
    })
}

#[derive(Clone, Copy)]
pub struct UseDepositState {
    inner: UseDepositInner,
}

#[derive(Clone, Copy, Default)]
pub struct UseDepositInner {
    deposit: Signal<DepositForm>,
}

impl UseDepositState {
    pub fn get(&self) -> UseDepositInner {
        self.inner
    }

    pub fn get_deposit(&self) -> DepositForm {
        self.inner.deposit.read().clone()
    }

    pub fn set_deposit(&mut self, deposit: DepositForm) {
        let mut inner = self.inner.deposit.write();
        *inner = deposit;
    }

    pub fn deposit_mut(&mut self) -> Signal<DepositForm> {
        self.inner.deposit
    }

    pub fn is_form_complete(&self) -> bool {
        let deposit = self.inner.deposit.read();

        deposit.is_valid()
    }

    pub fn default(&mut self) {
        self.inner = UseDepositInner::default()
    }
}
