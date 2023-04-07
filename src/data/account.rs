pub struct Account {
    user: String,
    pass: String,
}

impl Account {
    pub fn new(user: impl Into<String>, pass: impl Into<String>) -> Self {
        Self {
            user: user.into(),
            pass: pass.into(),
        }
    }

    pub fn user(&self) -> &str {
        &self.user
    }
    pub fn pass(&self) -> &str {
        &self.pass
    }
}

pub struct AccountManager {
    accounts: Vec<Account>,
}

impl AccountManager {
    pub fn new() -> Self {
        Self { accounts: vec![] }
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    pub fn check(&self, account: &Account) -> bool {
        for acc in &self.accounts {
            if acc.user == account.user && acc.pass == account.pass {
                return true;
            }
        }

        false
    }
}
