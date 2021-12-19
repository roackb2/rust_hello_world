use std::hash::{ Hash, Hasher };

#[derive(Debug)]
pub struct Transaction {
  sender: String,
  recipient: String,
  amount: i64,
}

impl Hash for Transaction {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.sender.hash(state);
    self.recipient.hash(state);
    self.amount.hash(state);
  }
}

impl Transaction {
  pub fn new(sender: String, recipient: String, amount: i64) -> Transaction {
    Transaction { sender, recipient, amount }
  }
}