mod transaction;

use std::{
    fs::File,
    io::{Read, Write},
};

use prettytable::Table;

use self::transaction::Transaction;
use crate::{question::Question, floating_decimal::FloatingPointDecimal};

/// A transaction registry
pub struct Registry {
    file_path: String,
    new_id: u64,
    accounts: Vec<String>,
    transactions: Vec<Transaction>,
}

impl Registry {
    /// Creates a registry from a file
    pub fn new(reg_path: &str) -> Result<Self, ()> {
        let file_path = reg_path.to_string();
        let mut new_id: u64 = 0;
        let mut accounts: Vec<String> = Vec::new();
        let transactions: Vec<Transaction>;

        // Open transactions file
        match File::open(&file_path) {
            Ok(mut f) => {
                let mut f_contents = String::new();
                f.read_to_string(&mut f_contents)
                    .expect("Unable to read the registry of transactions");

                match serde_json::from_str(&f_contents) {
                    Ok(loaded) => {
                        transactions = loaded;

                        for transaction in &transactions {
                            // Fill Account List
                            let from = transaction.get_from();
                            let to = transaction.get_to();

                            if !accounts.contains(&from) {
                                accounts.push(from);
                            }

                            if !accounts.contains(&to) {
                                accounts.push(to);
                            }

                            // Calculate New ID
                            let id = transaction.get_id();

                            if new_id < id {
                                new_id = id;
                            }
                            new_id += 1;
                        }

                        Ok(Self {
                            file_path,
                            new_id,
                            accounts,
                            transactions,
                        })
                    }
                    Err(_) => Err(()),
                }
            }

            Err(_) => {
                transactions = Vec::new();

                Ok(Self {
                    file_path,
                    new_id,
                    accounts,
                    transactions,
                })
            }
        }
    }

    /// Adds a new closed transaction promting the user through the CLI
    pub fn add_from_cli(&mut self) {
        let mut from: String;
        let mut to: String;

        loop {
            from = Question::new("Origin Account: ")
                .not_null()
                .not_containing(" ")
                .ask();

            if self.accounts.contains(&from) {
                break;
            } else {
                match Question::new("Would you like to add this new account? (Y/N) ")
                    .not_null()
                    .ask_yn()
                {
                    true => {
                        self.accounts.push(from.clone());
                        break;
                    }
                    false => (),
                }
            }
        }

        loop {
            to = Question::new("Destination Account: ")
                .not_null()
                .not_containing(" ")
                .ask();

            if self.accounts.contains(&to) {
                break;
            } else {
                match Question::new("Would you like to add this new account? (Y/N) ")
                    .not_null()
                    .ask_yn()
                {
                    true => {
                        self.accounts.push(to.clone());
                        break;
                    }
                    false => (),
                }
            }
        }

        let description = Question::new("Description: ").ask();

        let amount = Question::new("Digits: ")
            .not_null()
            .not_containing(".")
            .not_containing(",")
            .not_containing("$")
            .ask_floating_decimal();

        println!("${} will be sent from {} to {}.", amount, from, to);

        let new_transact = Transaction::new(
            self.generate_id(),
            None,
            from.clone(),
            to.clone(),
            amount,
            Some(description.clone()),
        );
        let closing_transact = Transaction::new(
            self.generate_id(),
            Some(new_transact.get_id()),
            from,
            to,
            amount,
            Some(description),
        );

        self.transactions.push(new_transact);
        self.transactions.push(closing_transact);
    }

    /// Adds a promise transaction promting the user through the CLI
    pub fn add_promise_cli(&mut self) {
        let mut from: String;
        let mut to: String;

        loop {
            from = Question::new("Origin Account: ")
                .not_null()
                .not_containing(" ")
                .ask();

            if self.accounts.contains(&from) {
                break;
            } else {
                match Question::new("Would you like to add this new account? (Y/N) ")
                    .not_null()
                    .ask_yn()
                {
                    true => {
                        self.accounts.push(from.clone());
                        break;
                    }
                    false => (),
                }
            }
        }

        loop {
            to = Question::new("Destination Account: ")
                .not_null()
                .not_containing(" ")
                .ask();

            if self.accounts.contains(&to) {
                break;
            } else {
                match Question::new("Would you like to add this new account? (Y/N) ")
                    .not_null()
                    .ask_yn()
                {
                    true => {
                        self.accounts.push(to.clone());
                        break;
                    }
                    false => (),
                }
            }
        }

        let description = Question::new("Description: ").ask();

        let amount = Question::new("Digits: ")
            .not_null()
            .not_containing(".")
            .not_containing(",")
            .not_containing("$")
            .ask_floating_decimal();

        println!("${} will be sent from {} to {}.", amount, from, to);

        let new_transact = Transaction::new(
            self.generate_id(),
            None,
            from.clone(),
            to.clone(),
            amount,
            Some(description.clone()),
        );

		self.transactions.push(new_transact)
    }

		/// Adds a promise payment transaction promting the user through the CLI
		pub fn add_payment_cli(&mut self) {
			// Select the promise to pay
			let cont_id: u64 = 0;
			loop {
				todo!("Code user input");
				break;
			}

			// Ask a valid (lower or equal to the remaining amount) amount
			loop {
				let amount = Question::new("Digits: ")
            .not_null()
            .not_containing(".")
            .not_containing(",")
            .not_containing("$")
            .ask_floating_decimal();
				
				// Check if its less or equal to the remaining amount
				if amount <= self.calculate_promise_remaining_amount(cont_id).expect("Expected an existing ID") {
					break;
				}
			}

			// Save the transaction
		}

    /// Saves the current state to the registry file
    pub fn save(&self) {
        let mut f = File::create(&self.file_path).expect("Unable to rewrite the file");
        let json = serde_json::to_string(&self.transactions).expect("Error serializing");

        f.write_all(json.as_bytes())
            .expect("Unable to rewrite the file");
    }

    /// Displays all the transactions in console
    pub fn show_transactions(&self) {
        println!("FULL TRANSACTION REGISTRY");

        let mut table = Table::new();
        table.set_titles(row![bc => "ID", "DATE", "DESCRIPTION", "FROM", "TO", "TYPE", "AMOUNT"]);

        for transaction in &self.transactions {
            transaction.print_row(&mut table);
        }

        table.printstd();
        println!("")
    }

		/// Returns a copy of the requested transaction
		pub fn getTransaction(&self, transaction_id: u64) -> Result<Transaction, &str> {
			let mut result = Err("Transaction ID Not Found");

			for transaction in &self.transactions {
				if transaction.get_id()==transaction_id {
					result = Ok(transaction.clone());
					break;
				}
			}

			result
		}

		/// Returns the amount remaining to fully pay a promise
		pub fn calculate_promise_remaining_amount(&self, promise_id: u64) -> Result<FloatingPointDecimal, &str> {
			match self.getTransaction(promise_id) {
				Ok(promise) => {
					let mut remaining_amount = promise.get_money();

					// For each continuation transaction, substracts its value
					for transaction in &self.transactions {
						match transaction.get_continue() {
							Some(cont_id) => {
								if promise.get_id()==cont_id {
									// Substract
									remaining_amount = remaining_amount-transaction.get_money();
								}
							},
							None => ()
						}
					}

					Ok(remaining_amount)
				},
				Err(error) => Err(error)
			}
		}

    /// Generates a new ID an updates the ID count
    fn generate_id(&mut self) -> u64 {
        self.new_id += 1;

        self.new_id - 1
    }
}
