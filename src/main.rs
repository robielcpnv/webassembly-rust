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
fn is_verified(account: &BankAccount) -> Result<bool, bool> {
    match account.verified {
        true => Ok(true),
        false => Err(false),
        
    }    
}
fn main() {
    let my_account = BankAccount {
        balance: 100,
        verified: true,
        owner: String::from("Robiel Tesfazghi"),
    };
    let verification_status = is_verified(&my_account)
        .expect("Error checking verification status");
    print_balance(&my_account);
    print_verified(&my_account);
    print_owner(&my_account);
    println!("Account verification status: {:?}", verification_status);
}
