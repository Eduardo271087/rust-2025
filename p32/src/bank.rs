trait UserInfo {
    fn get_name(&self) -> String;
    fn set_name(&mut self, name: String);
    fn get_credit_line(&self) -> u64;
    fn set_credit_line(&mut self, credit_line: u64);
    fn get_balance(&self) -> i64;
    fn set_balance(&mut self, balance: i64);
}
#[derive(Debug, Clone)]
struct User {
    name: String,
    credit_line: u64,
    balance: i64,
}

trait BankInfo {
    fn calc_balance(&self) -> (u64, i64);
    fn get_users(&self) -> Vec<User>;
    fn set_users(&mut self, users: Vec<User>);
    fn get_name(&self) -> String;
    fn set_name(&mut self, name: String);
    fn get_credit_interest(&self) -> u64;
    fn set_credit_interest(&mut self, credit_interest: u64);
    fn get_debit_interest(&self) -> u64;
    fn set_debit_interest(&mut self, debit_interest: u64);
}

#[derive(Debug)]
struct Bank {
    users: Vec<User>,
    name: String,
    credit_interest: u64,
    debit_interest: u64,
}

impl UserInfo for User {
    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn get_credit_line(&self) -> u64 {
        self.credit_line
    }
    fn set_credit_line(&mut self, credit_line: u64) {
        self.credit_line = credit_line;
    }

    fn get_balance(&self) -> i64 {
        self.balance
    }

    fn set_balance(&mut self, balance: i64) {
        self.balance = balance;
    }
}

impl User {
    pub fn new(name: String, credit_line: u64, balance: i64) -> Self {
        Self {
            name,
            credit_line,
            balance,
        }
    }
}

impl BankInfo for Bank {
    fn calc_balance(&self) -> (u64, i64) {
        let assets = self.users.iter().map(|u| u.credit_line).sum();
        let liabilities = self.users.iter().map(|u| u.balance).sum();

        (assets, liabilities)
    }

    fn get_users(&self) -> Vec<User> {
        self.users.to_vec()
    }

    fn set_users(&mut self, users: Vec<User>) {
        self.users = users;
    }

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn get_credit_interest(&self) -> u64 {
        self.credit_interest
    }

    fn set_credit_interest(&mut self, credit_interest: u64) {
        self.credit_interest = credit_interest;
    }

    fn get_debit_interest(&self) -> u64 {
        self.debit_interest
    }

    fn set_debit_interest(&mut self, debit_interest: u64) {
        self.debit_interest = debit_interest;
    }
}

impl Bank {
    pub fn new(users: Vec<User>, name: String, credit_interest: u64, debit_interest: u64) -> Self {
        Self {
            users,
            name,
            credit_interest,
            debit_interest,
        }
    }

    // fn get_user_balance(username: String) -> i64 {}

    // pub fn transfer_funds(
    //     source_user: String,
    //     destination_user: String,
    //     transfer_amount: u64,
    // ) -> i32 {
    // }
}
