mod bank_account;

fn main() {
    let mut account = bank_account::BankAccount::new(100.0);
    println!("Initial balance: {}", account.balance());

    account.deposit(50.0);
    println!("After depositing 50: {}", account.balance());

    account.withdraw(30.0);
    println!("After withdrawing 30: {}", account.balance());

    account.withdraw(200.0);
    println!("After attempting to withdraw 200: {}", account.balance());

    account.deposit(-10.0);
    println!("After attempting to deposit -10: {}", account.balance());
}
