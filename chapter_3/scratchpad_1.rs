// A fully compilable file for woking out details

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

  pub fn set_contract_disabled_for_customer(&mut self, customer_id: usize) {
    // need to use &mut here to explicitly say the item will be modified
    // in the loop body
    for customer in &mut self.all_customers {
      if customer.id == customer_id {
        customer.contract.enabled = false;
      }
    }
  }

  pub fn get_customer_by_id(&mut self, customer_id: usize) -> Vec<&mut Customer> {
    let mut outlist: Vec<&mut Customer> = vec!();
    for customer in &mut self.all_customers {
      if customer.id == customer_id {
        outlist.push(customer)
      }
    }
    outlist
  }

  pub fn filter<T>(&mut self, test: T) -> Vec<&mut Customer>
    where T: Fn(&Customer) -> bool {
    let mut outlist: Vec<&mut Customer> = vec!();
    for customer in &mut self.all_customers {
      if test(customer) {
        outlist.push(customer)
      }
    }
    outlist
  }

  pub fn get_field<'a, B, F>(&'a self,
                             test: fn(&Customer) -> bool,
                             func: F) -> Vec<B>
    where F: Fn(&'a Customer) -> B {

    let mut outlist: Vec<B> = vec!();
    for customer in self.all_customers.iter() {
      if test(customer) {
        outlist.push(func(customer));
      }
    }
    outlist
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

  pub name: String,
  pub address: String,
  pub state: String,
  pub primary_contact: String,
  pub domain: String,
  pub enabled: bool
}

fn main() {
  // something
}
