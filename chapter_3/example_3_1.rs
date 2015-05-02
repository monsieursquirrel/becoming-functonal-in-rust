extern crate chrono;

use chrono::{Date, Local, Duration};

pub struct Contract {
  // Date<Local> is a date in the local timezone
  start_date: Date<Local>,
  end_date: Date<Local>,
  enabled: bool,
}

impl Contract {
  pub fn new(begin_date: Date<Local>) -> Contract {
    Contract {
      start_date: begin_date,
      end_date: begin_date + Duration::days(2 * 365),
      enabled: true,
    }
  }
}

impl AllCustomers {

  // the most logical place for this function to be in a Rust program would be
  // attached to AllCustomers

  fn set_contract_disabled_for_customer(&mut self, customer_id: usize) {
    // need to use &mut here to explicitly say the item will be modified
    // in the loop body
    for customer in &mut self.all_customers {
      if customer.id == customer_id {
        customer.contract.enabled = false;
      }
    }
  }
}


// --------------- stuff needed by the example code above ----------------
pub struct AllCustomers {
  pub all_customers: Vec<Customer>,
  pub id: usize,
}

pub struct Customer {
  id: usize,
  contract: Contract,
}

fn main() {
  // something
}
