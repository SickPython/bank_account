#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount {
            balance: if initial_balance < 0.0 { 0.0 } else { initial_balance },
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_new_account() {
        let account = BankAccount::new(100.0);
        assert!((account.balance() - 100.0).abs() < EPSILON, "Balance should be 100.0");

        let account = BankAccount::new(-50.0);
        assert!((account.balance() - 0.0).abs() < EPSILON, "Balance should be 0.0 for negative initial balance");
    }

    #[test]
    fn test_deposit() {
        let mut account = BankAccount::new(50.0);
        account.deposit(30.0);
        assert!((account.balance() - 80.0).abs() < EPSILON, "Balance should be 80.0 after depositing 30.0");

        account.deposit(-10.0);
        assert!((account.balance() - 80.0).abs() < EPSILON, "Balance should remain 80.0 after negative deposit");
    }

    #[test]
    fn test_withdraw() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(50.0);
        assert!((account.balance() - 50.0).abs() < EPSILON, "Balance should be 50.0 after withdrawing 50.0");

        account.withdraw(200.0);
        assert!((account.balance() - 50.0).abs() < EPSILON, "Balance should remain 50.0 after attempting to withdraw more than the balance");

        account.withdraw(-10.0);
        assert!((account.balance() - 50.0).abs() < EPSILON, "Balance should remain 50.0 after attempting negative withdrawal");
    }

    #[test]
    fn test_balance() {
        let account = BankAccount::new(200.0);
        assert!((account.balance() - 200.0).abs() < EPSILON, "Balance should be 200.0");
    }

    #[test]
    fn test_withdraw_all_funds() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(100.0);
        assert!((account.balance() - 0.0).abs() < EPSILON, "Balance should be 0.0 after withdrawing all funds");
    }
}