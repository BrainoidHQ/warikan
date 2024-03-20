use crate::entities::{Payment, UserID};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(test)]
use fake::Dummy;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(test, derive(Dummy))]
pub struct Warikan {
    pub from: UserID,
    pub to: UserID,
    pub amount: i32,
}

pub fn warikan(payments: &[Payment]) -> Option<Vec<Warikan>> {
    let mut balance = HashMap::new();
    for payment in payments.iter() {
        for creditor in payment.creditors.iter() {
            *balance.entry(&creditor.user).or_insert(0) += creditor.amount;
        }
        for debtor in payment.debtors.iter() {
            *balance.entry(&debtor.user).or_insert(0) -= debtor.amount;
        }
    }

    if balance.values().sum::<i32>() != 0 {
        return Some(Vec::new()); // TODO
    }
    if balance.len() < 2 {
        return Some(Vec::new());
    }

    let mut warikans = Vec::new();
    let mut balance = balance
        .into_iter()
        .map(|(user, amount)| (amount, user))
        .collect::<Vec<_>>();
    balance.sort();

    while balance[0].0 != 0 {
        let (debt, credit) = (balance.first().unwrap(), balance.last().unwrap());
        let amount = debt.0.abs().min(credit.0.abs());
        warikans.push(Warikan {
            from: debt.1.clone(),
            to: credit.1.clone(),
            amount,
        });
        balance.first_mut().unwrap().0 += amount;
        balance.last_mut().unwrap().0 -= amount;
        balance.sort();
    }

    Some(warikans)
}
