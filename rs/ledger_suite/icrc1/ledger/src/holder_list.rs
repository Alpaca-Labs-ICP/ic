use crate::HOLDER_STORE;
use candid::{CandidType, Nat};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Debug, Clone, Serialize)]
pub struct HolderListMetadata {
    pub total: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone, Serialize)]
pub struct HolderData {
    pub account: Account,
    pub amount: Nat,
    pub percentage: f64,
}

#[derive(CandidType, Deserialize, Debug, Clone, Serialize)]
pub struct HolderListResp {
    pub metadata: HolderListMetadata,
    pub data: Vec<HolderData>,
}

#[derive(CandidType, Deserialize, Debug, Clone, Serialize)]
pub struct UpsertHolderInput {
    pub account: Account,
    pub amount: u64,
}

pub fn upsert_holders(input: Vec<UpsertHolderInput>) {
    ic_cdk::print(format!("upsert_holders: {:?}", input));
    HOLDER_STORE.with_borrow_mut(|list| {
        for holder in input {
            list.insert(holder.account, holder.amount);
        }
    })
}

pub fn get_holders(offset: u32, limit: u32, total_supply: u64) -> HolderListResp {
    let mut data = vec![];
    let mut total = 0;

    HOLDER_STORE.with_borrow(|list| {
        total = list.len() as u64;

        let mut sorted_list: Vec<_> = list.iter().collect();
        sorted_list.sort_by(|a, b| b.1.cmp(&a.1)); // Sort in descending order by amount

        // Paginate the sorted list
        let paginated_list = sorted_list
            .iter()
            .skip(offset as usize)
            .take(limit as usize);

        for (account, amount) in paginated_list {
            let percentage = (*amount as f64) / (total_supply as f64);
            data.push(HolderData {
                account: account.clone(),
                amount: Nat::from(*amount),
                percentage,
            });
        }
    });

    HolderListResp {
        metadata: HolderListMetadata { total },
        data,
    }
}

pub fn count_holders() -> u64 {
    let mut total = 0;
    HOLDER_STORE.with_borrow(|list| {
        total = list.len() as u64;
    });
    total
}
