#[derive(Debug)]
struct Date (u32, u32, u32);

#[derive(Debug)]
enum TransferDirection {
  Send,
  Receive
}

#[derive(Debug)]
struct Transaction {
  amount: u32,
  attendant: String,
  direction: TransferDirection,
  done: bool
}

impl Transaction {
  fn new(amount: u32, attendant: String, direction: TransferDirection) -> Transaction {
    Transaction {
      amount,
      attendant,
      direction,
      done: false
    }
  }
}

#[derive(Debug)]
struct ScheduledPayment {
  amount: u32,
  attendant: String,
  direction: TransferDirection,
  due_date: Date
}

#[derive(Debug)]
struct File {
  document_id: String,
  content: String,
  attachments: Option<Vec<String>>
}

impl File {
  fn empty() -> File {
    File {
      document_id: String::from(""),
      content: String::from(""),
      attachments: None
    }
  }
  fn add_attachment(&mut self, file_name: String) {
    if let Some(attachments) = self.attachments.as_mut() {
      attachments.push(file_name);
    }
  }
}


#[derive(Debug)]
enum Transfer {
  Payment(Transaction),
  Loan(ScheduledPayment),
  Invoice(File),
  Noop,
}

impl Transfer {
  fn kind(&self) -> &str {
    match self {
      Transfer::Payment(_) => "This is a payment",
      Transfer::Loan(_) => "This is a loan",
      Transfer::Invoice(_) => "This is a invoice",
      Transfer::Noop => "Nothing happened"
    }
  }
  fn action(&mut self) {
    match self {
      Transfer::Payment(trx) => {
        trx.done = true;
      }
      _ => {}
    }
  }
  fn add_attachment(&mut self, file_name: String) {
    match self {
      Transfer::Invoice(inv) => {
        inv.add_attachment(file_name);
      }
      _ => {}
    }
  }
}

pub fn test_enums() {
  let mut to_alice = Transfer::Payment(
    Transaction::new(32, String::from("alice@company.com"), TransferDirection::Send)
  );
  println!("A transaction to Alice: {:#?}", to_alice);
  println!("What kind is this transfer? {}", to_alice.kind());
  to_alice.action();
  println!("The payment has been made! {:#?}", to_alice);

  let received = Transfer::Payment(
    Transaction::new(100, String::from("bob@company.com"), TransferDirection::Receive)
  );

  if let Transfer::Payment(trx) = received {
    println!("Received {} dolors from {}", trx.amount, trx.attendant)
  }

  let lend_me = Transfer::Loan(
    ScheduledPayment {
      amount: 80,
      attendant: String::from("john@company.com"),
      due_date: Date(2022, 01, 03),
      direction: TransferDirection::Receive
    }
  );
  println!("A Loan {:#?}", lend_me);

  let mut files = Vec::new();
  files.push(String::from("some_file.pdf"));
  let mut my_invoice = Transfer::Invoice(
    File {
      document_id: String::from("doc_0013"),
      content: String::from("I received 30 dolors from Alice"),
      attachments: Some(files),
    }
  );
  println!("A invoice: {:#?}", my_invoice);

  my_invoice.add_attachment(String::from("my_pic.png"));
  println!("Updated invoice: {:#?}", my_invoice);
}
