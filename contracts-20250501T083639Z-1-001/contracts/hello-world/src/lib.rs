#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Symbol, symbol_short};

// Structure to represent a loan request
#[contracttype]
#[derive(Clone)]
pub struct Loan {
    pub loan_id: u64,
    pub borrower: Address,
    pub lender: Option<Address>,
    pub amount: i128,
    pub repaid: bool,
    pub timestamp: u64,
}

// Storage keys
#[contracttype]
pub enum LoanKey {
    Loan(u64),
    Count,
}

#[contract]
pub struct P2PLendingApp;

#[contractimpl]
impl P2PLendingApp {
    // Create a new loan request
    pub fn request_loan(env: Env, borrower: Address, amount: i128) -> u64 {
        let mut count = env.storage().instance().get(&LoanKey::Count).unwrap_or(0);
        count += 1;

        let loan = Loan {
            loan_id: count,
            borrower,
            lender: None,
            amount,
            repaid: false,
            timestamp: env.ledger().timestamp(),
        };

        env.storage().instance().set(&LoanKey::Loan(count), &loan);
        env.storage().instance().set(&LoanKey::Count, &count);

        count
    }

    // Fund a loan as a lender
    pub fn fund_loan(env: Env, loan_id: u64, lender: Address) {
        let mut loan: Loan = env.storage().instance().get(&LoanKey::Loan(loan_id)).expect("Loan not found");

        if loan.lender.is_some() {
            panic!("Loan already funded");
        }

        loan.lender = Some(lender);
        env.storage().instance().set(&LoanKey::Loan(loan_id), &loan);
    }

    // Repay a loan
    pub fn repay_loan(env: Env, loan_id: u64) {
        let mut loan: Loan = env.storage().instance().get(&LoanKey::Loan(loan_id)).expect("Loan not found");

        if loan.repaid {
            panic!("Loan already repaid");
        }

        loan.repaid = true;
        env.storage().instance().set(&LoanKey::Loan(loan_id), &loan);
    }

    // View loan details
    pub fn view_loan(env: Env, loan_id: u64) -> Loan {
        env.storage().instance().get(&LoanKey::Loan(loan_id)).expect("Loan not found")
    }
}
