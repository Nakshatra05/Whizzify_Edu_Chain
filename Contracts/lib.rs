use soroban_sdk::{contractimpl, Env, Address, Symbol};

pub struct FundingContract;

#[contractimpl]
impl FundingContract {
    pub fn fund(env: Env, amount: u64) {
        let contract_address = env.contract_address().unwrap();
        let balance: u64 = env.balance_of(&contract_address);
        env.assert(balance >= amount, "Insufficient contract balance");
        // Transfer the amount to the contract's balance
        env.transfer(&contract_address, &env.sender(), amount).unwrap();
    }

    pub fn disburse(env: Env, recipient: Address, amount: u64) {
        let contract_address = env.contract_address().unwrap();
        let balance: u64 = env.balance_of(&contract_address);
        env.assert(balance >= amount, "Insufficient contract balance");
        // Transfer the amount to the recipient
        env.transfer(&contract_address, &recipient, amount).unwrap();
    }
}

#[cfg(test)]
mod test;
