// A fully compilable file for woking out details

extern crate chrono;

use chrono::{Date, Local, Duration};

pub struct Contract {
  // Date<Local> is a date in the local timezone
  begin_date: Date<Local>,
  end_date: Date<Local>,
  enabled: bool,
}

impl Contract {
  pub fn new(begin_date: Date<Local>) -> Contract {
    Contract {
      begin_date: begin_date,
      end_date: begin_date + Duration::days(2 * 365),
      enabled: true,
    }
  }
}

impl Contract {
  pub fn set_begin_date(&mut self, begin_date: Date<Local>) -> &mut Self {
    self.begin_date = begin_date;
    self
  }

  pub fn set_end_date(&mut self, end_date: Date<Local>) -> &mut Self {
    self.end_date = end_date;
    self
  }

  pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
    self.enabled = enabled;
    self
  }
}

impl Customer {
  pub fn set_customer_id(&mut self, id: usize) -> &mut Self {
    self.id = id;
    self
  }

  pub fn set_name(&mut self, name: String) -> &mut Self {
    self.name = name;
    self
  }

  pub fn set_state(&mut self, state: String) -> &mut Self {
    self.state = state;
    self
  }

  // ... you get the idea ...
}

impl AllCustomers {

  pub fn get_field<'a, B, F>(&'a mut self,
                             test: fn(&Customer) -> bool,
                             func: F) -> Vec<B>
    where F: Fn(&'a Customer) -> B {

    let mut outlist: Vec<B> = vec!();
    for customer in filter(&mut self.all_customers, test) {
      outlist.push(func(customer));
    }
    outlist
  }

  pub fn set_contract_for_customer(inlist: &mut Vec<Customer>,
                                   customer_id: usize,
                                   status: bool) {
    map(get_customer_by_id(inlist, customer_id), |customer| {
      customer.contract.set_enabled(status)
    });
  }

}

pub fn filter<T>(inlist: &mut Vec<Customer>, test: T) -> Vec<&mut Customer>
  where T: Fn(&Customer) -> bool {
  let mut outlist: Vec<&mut Customer> = vec!();
  for customer in inlist {
    if test(customer) {
      outlist.push(customer)
    }
  }
  outlist
}

pub fn map<T, A, B>(inlist: Vec<A>, func: T) -> Vec<B>
  where T: Fn(A) -> B {
  let mut outlist: Vec<B> = vec!();
  for obj in inlist {
    outlist.push(func(obj))
  }
  outlist
}


pub fn for_each<F>(inlist: Vec<&mut Customer>, func: F)
  where F: Fn(&mut Customer){
  for customer in inlist {
    func(customer);
  }
}

pub fn get_customer_by_id(inlist: &mut Vec<Customer>, customer_id: usize) -> Vec<&mut Customer> {
  return filter(inlist, |customer| { customer.id == customer_id })
}

pub fn set_contract_enabled_for_customer(inlist: &mut Vec<Customer>, customer_id: usize) {
  for_each(get_customer_by_id(inlist, customer_id), |customer| {
    customer.contract.enabled = true;
  });
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
