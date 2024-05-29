// vault pjs

extern crate alloc;
use alloc::vec::Vec;
use libwallet::{any::AnySignature, Account, Public, Signer, Vault};
use sp_core::crypto::{AccountId32, Ss58Codec};
#[derive(Clone, Debug)]
pub struct Pjs {
    inner: PjsExtension,
}

impl Pjs {
    pub async fn connect(name: &str) -> Result<Self, Error> {
        Ok(Pjs {
            inner: PjsExtension::connect(name).await?,
        })
    }

    pub async fn list_accounts(&mut self) -> Result<Vec<PjsAccount>, Error> {
        self.inner.fetch_accounts().await?;
        Ok(self.inner.accounts())
    }

    pub fn select_account(&mut self, idx: u8) {
        self.inner.select_account(idx)
    }
}

impl Signer for Pjs {
    type Signature = AnySignature;

    async fn sign_msg(&self, msg: impl AsRef<[u8]>) -> Result<Self::Signature, ()> {
        let sig = self.inner.sign(msg.as_ref()).await.map_err(|_| ())?;
        Ok(AnySignature::from(sig))
    }

    async fn verify(&self, _: impl AsRef<[u8]>, _: impl AsRef<[u8]>) -> bool {
        unimplemented!()
    }
}

impl Account for Pjs {
    fn public(&self) -> impl Public {
        let mut key = [0u8; 32];
        let address = self
            .inner
            .get_selected()
            .expect("an account must be defined")
            .address();

        let address = <AccountId32 as Ss58Codec>::from_string(&address).expect("it must be a valid ss58 address");
        log::info!("{:?}", address);
        // let pub_key = hex::decode(address.as_slice()).expect("it match");

        key.copy_from_slice(address.as_ref());
        key
    }
}

impl core::fmt::Display for Pjs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        for byte in self.public().as_ref() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl Vault for Pjs {
    type Id = u8;
    type Credentials = ();
    type Account = Pjs;
    type Error = Error;

    async fn unlock(
        &mut self,
        account: Self::Id,
        _: impl Into<Self::Credentials>,
    ) -> Result<Self::Account, Self::Error> {
        let mut pjs_signer = self.clone();
        pjs_signer.select_account(account);
        Ok(pjs_signer)
    }
}

// pjs-rs

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::{Array, Function, Object, Promise, Reflect};

macro_rules! get {
    (^ $obj:expr, $($prop:expr),+ $(,)?) => {{
        let val = get!($obj, $($prop),+);
        val.unchecked_into()
    }};
    ($obj:expr, $($prop:expr),+ $(,)?) => {{
        let mut current_val = JsValue::from($obj);
        $(
            current_val = Reflect::get(&current_val, &JsValue::from_str($prop))
                .unwrap_or_else(|_| panic!("Property '{}' does not exist in {:?}", $prop, current_val));
        )+
        current_val
    }};
}

const NULL: JsValue = JsValue::null();

#[derive(Clone, Debug)]
pub struct PjsExtension {
    pjs: JsValue,
    accounts: Vec<PjsAccount>,
    selected: Option<u8>,
}

impl PjsExtension {
    pub async fn connect(app_name: &str) -> Result<PjsExtension, Error> {
        let Some(web3) = web_sys::window().expect("browser").get("injectedWeb3") else {
            return Err(Error::ExtensionUnavailable);
        };
        let pjs = get!(web3, "polkadot-js");
        let enable: Function = get!(^ &pjs, "enable");
        let p = enable
            .call1(&pjs, &app_name.into())
            .expect("promise")
            .unchecked_into::<Promise>();
        let Ok(pjs) = JsFuture::from(p).await else {
            return Err(Error::NoPermission);
        };

        Ok(Self {
            pjs,
            accounts: vec![],
            selected: None,
        })
    }

    pub fn select_account(&mut self, idx: u8) {
        self.selected = self
            .accounts
            .len()
            .checked_sub(1)
            .map(|i| idx.min(i.min(u8::MAX as usize) as u8));
    }

    ///

    pub async fn js_sign(&self, payload: &str, cb: &Function) -> Result<JsValue, Error> {
        let sign: Function = get!(^ &self.pjs, "signer", "signRaw");
        let account = self
            .accounts
            .get(self.selected.ok_or(Error::NoAccountSelected)? as usize)
            .ok_or(Error::NoAccounts)?;
        let data = {
            let o = Object::new();
            Reflect::set(&o, &"address".into(), &account.address.as_str().into()).unwrap();
            Reflect::set(&o, &"data".into(), &payload.into()).unwrap();
            Reflect::set(&o, &"type".into(), &"bytes".into()).unwrap();
            o
        };

        let p = sign
            .call1(&NULL, &data.into())
            .expect("promise")
            .unchecked_into::<Promise>();
        let signature = JsFuture::from(p).await.map_err(|_| Error::Sign)?;
        let res = cb.call1(&NULL, &signature).map_err(|_| Error::Sign)?;
        Ok(get!(&res, "signature"))
    }

    ///

    pub async fn fetch_accounts(&mut self) -> Result<(), Error> {
        let accounts: Function = get!(^ &self.pjs, "accounts", "get");
        let p = accounts.call0(&NULL).unwrap().unchecked_into::<Promise>();
        let Ok(accounts) = JsFuture::from(p).await else {
            return Err(Error::FailedFetchingAccounts);
        };
        self.accounts = Array::from(&accounts)
            .iter()
            .map(|a| {
                let name = get!(&a, "name").as_string().unwrap();
                let address = get!(&a, "address").as_string().unwrap();
                let net: Network = get!(&a, "genesisHash").into();
                PjsAccount { name, address, net }
            })
            .collect();
        if !self.accounts.is_empty() {
            self.selected = Some(0);
        }
        Ok(())
    }

    pub fn accounts(&self) -> Vec<PjsAccount> {
        self.accounts.clone()
    }

    pub fn get_selected(&self) -> Option<PjsAccount> {
        self.selected
            .and_then(|a| self.accounts.get(a as usize))
            .cloned()
    }
}

impl PjsExtension {
    pub async fn sign(&self, payload: &[u8]) -> Result<[u8; 64], Error> {
        let payload = Self::to_hex(payload);
        let mut signature = [0u8; 64];
        let cb = Closure::wrap(Box::new(move |s: JsValue| {
            Self::from_hex(s.as_string().unwrap_or_default().as_str(), &mut signature)
        }) as Box<dyn FnMut(JsValue)>);
        self.js_sign(payload.as_str(), cb.as_ref().unchecked_ref())
            .await?;
        Ok(signature)
    }

    fn to_hex(bytes: &[u8]) -> String {
        use std::fmt::Write;
        let mut s = String::with_capacity(2 + bytes.len());
        let _ = write!(s, "0x");
        for b in bytes {
            let _ = write!(s, "{b:x}");
        }
        s
    }
    fn from_hex(input: &str, buf: &mut [u8]) {
        for (i, b) in buf.iter_mut().enumerate() {
            let Some(s) = input.get(i * 2..i * 2 + 2) else {
                return;
            };
            *b = u8::from_str_radix(s, 16).unwrap_or_default();
        }
    }
}

// #[wasm_bindgen]
#[derive(Debug)]
pub enum Error {
    ExtensionUnavailable,
    NoPermission,
    FailedFetchingAccounts,
    NoAccountSelected,
    NoAccounts,
    Sign,
}

// #[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct PjsAccount {
    name: String,
    address: String,
    net: Network,
}

impl PjsAccount {
    pub fn new(name: &str, address: &str, net: Network) -> Self {
        PjsAccount {
            name: name.to_string(),
            address: address.to_string(),
            net,
        }
    }
    // #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }
    // #[wasm_bindgen(getter)]
    pub fn address(&self) -> String {
        self.address.clone()
    }
    // #[wasm_bindgen(getter)]
    pub fn network(&self) -> Network {
        self.net
    }
}

// #[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Network {
    Generic,
    Kusama,
    Polkadot,
    Kreivo,
}

const KSM: &str = "0xb0a8d493285c2df73290dfb7e61f870f17b41801197a149ca93654499ea3dafe";
const DOT: &str = "0x91b171bb158e2d3848fa23a9f1c25182fb8e20313b2c1eb49219da7a70ce90c3";
const KREIVO: &str = "0xc710a5f16adc17bcd212cff0aedcbf1c1212a043cdc0fb2dcba861efe5305b01";

impl From<JsValue> for Network {
    fn from(value: JsValue) -> Self {
        let value = value.as_string();
        match value.as_deref() {
            Some(KSM) => Network::Kusama,
            Some(DOT) => Network::Polkadot,
            Some(KREIVO) => Network::Kreivo,
            _ => Network::Generic,
        }
    }
}
