struct BankAccount {
    balance: i32,
    verified: bool,
    owner: String,
}
fn print_balance(account: &BankAccount) {
    println!("Account balance: {:?}", account.balance);
}
fn print_verified(account: &BankAccount) {
    println!("Account verified: {:?}", account.verified);
}
fn print_owner(account: &BankAccount) {
    println!("Account owner: {:?}", account.owner);
}
fn main() {
    let my_account = BankAccount {
        balance: 100,
        verified: true,
        owner: String::from("Robiel Tesfazghi"),
    };
    print_balance(&my_account);
    print_verified(&my_account);
    print_owner(&my_account);
}
