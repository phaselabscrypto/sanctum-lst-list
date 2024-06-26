use serde::{Deserialize, Serialize};
use serde_with::{serde_as, As, DisplayFromStr};
use solana_program::pubkey::Pubkey;

use crate::PoolProgram;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(tag = "program")]
pub enum PoolInfo {
    Lido,
    Marinade,
    ReservePool,
    SanctumSpl(SplPoolAccounts),
    Spl(SplPoolAccounts),
    SPool(SPoolAccounts),
    SanctumSplMulti(SplPoolAccounts),
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SplPoolAccounts {
    #[serde(with = "As::<DisplayFromStr>")]
    pub pool: Pubkey,

    #[serde(with = "As::<DisplayFromStr>")]
    pub validator_list: Pubkey,

    #[serde(default)]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub vote_account: Option<Pubkey>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SPoolAccounts {
    #[serde(with = "As::<DisplayFromStr>")]
    pub program_id: Pubkey,
}

impl PoolInfo {
    pub fn pool_program(&self) -> PoolProgram {
        match self {
            PoolInfo::Lido => PoolProgram::Lido,
            PoolInfo::Marinade => PoolProgram::Marinade,
            PoolInfo::ReservePool => PoolProgram::ReservePool,
            PoolInfo::SanctumSpl(..) => PoolProgram::SanctumSpl,
            PoolInfo::Spl(..) => PoolProgram::Spl,
            PoolInfo::SPool(..) => PoolProgram::InfSPool,
            PoolInfo::SanctumSplMulti(..) => PoolProgram::SanctumSplMulti,
        }
    }
}

/// The entry for a single supported LST
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SanctumLst {
    #[serde(with = "As::<DisplayFromStr>")]
    pub mint: Pubkey,

    #[serde(with = "As::<DisplayFromStr>")]
    pub token_program: Pubkey,

    pub name: String,
    pub symbol: String,
    pub logo_uri: String,
    pub decimals: u8,
    pub pool: PoolInfo,
}
