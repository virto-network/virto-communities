use crate::hooks::use_connect_wallet::Wallet;

pub enum DecisionMethod {
    Rank,
    CommunityAsset,
    NativeToken,
    Membership,
}

pub async fn register(
    wallet: Wallet,
    community_id: u16,
    name: &str,
    account: u32,
    decision: DecisionMethod,
) {
    let query = format!("wss://kreivo.kippu.rocks/communitiesManger/register");
}
