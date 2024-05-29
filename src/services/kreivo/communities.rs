use crate::hooks::use_connect_wallet::Wallet;

pub enum MultiAddress {
    Id(u32),
    Index(u16),
    Raw(String),
    Address32(u32),
    Address20(u32),
}

pub async fn transferKeepAlive(wallet: Wallet, who: MultiAddress) {
    let query = format!("wss://kreivo.kippu.rocks/communities/addMember");
}
