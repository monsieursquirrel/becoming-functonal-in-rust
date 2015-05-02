// nearly everything so far

extern crate chrono;

use chrono::{Date, Local, Duration};

#[derive(Debug)]
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

pub struct Customer {
  id: usize,
  name: String,
  address: String,
  state: String,
  primary_contact: String,
  domain: String,
  enabled: bool,
  contract: Contract,
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

  pub fn set_address(&mut self, address: String) -> &mut Self {
    self.address = address;
    self
  }

  pub fn set_state(&mut self, state: String) -> &mut Self {
    self.state = state;
    self
  }

  pub fn set_primary_contact(&mut self, primary_contact: String) -> &mut Self {
    self.primary_contact = primary_contact;
    self
  }

  pub fn set_domain(&mut self, domain: String) -> &mut Self {
    self.domain = domain;
    self
  }

  pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
    self.enabled = enabled;
    self
  }

  pub fn set_contract(&mut self, contract: Contract) -> &mut Self {
    self.contract = contract;
    self
  }

  // useful filter functions - double references for use in iterators
  pub fn enabled_customer(customer: &&Customer) -> bool { customer.enabled }
  pub fn disabled_customer(customer: &&Customer) -> bool { !customer.enabled }
}

pub struct AllCustomers {
  all_customers: Vec<Customer>,
}

impl AllCustomers {
  pub fn new() -> Self {
    AllCustomers {
      all_customers: vec!(),
    }
  }

  pub fn get_disabled_customer_names(&self) -> Vec<&str> {
    self.all_customers.iter()
      .filter(Customer::disabled_customer)
      .map(|customer| { &customer.name[..] })
      .collect()
  }

  pub fn get_enabled_customer_states(&self) -> Vec<&str> {
    self.all_customers.iter()
      .filter(Customer::enabled_customer)
      .map(|customer| { &customer.state[..] })
      .collect()
  }

  pub fn get_enabled_customer_domains(&self) -> Vec<&str> {
    self.all_customers.iter()
      .filter(Customer::enabled_customer)
      .map(|customer| { &customer.domain[..] })
      .collect()
  }

  pub fn get_enabled_customer_someone_email(&self, someone: &str) -> Vec<String> {
    self.all_customers.iter()
      .filter(Customer::enabled_customer)
      .map(|customer| {
        let mut email = String::new();
        email.push_str(someone);
        email.push_str("@");
        email.push_str(&customer.domain);
        email
      })
      .collect()
  }

  pub fn get_customer_by_id(&self, customer_id: usize) -> Vec<&Customer> {
    self.all_customers.iter()
      .filter(|customer| { customer.id == customer_id })
      .collect()
  }

  pub fn set_contract_for_customer(&mut self,
                                   customer_id: usize,
                                   status: bool) {
    self.all_customers.iter_mut()
      .filter(|customer| { customer.id == customer_id })
      .map(|customer| { customer.contract.set_enabled(status) })
      .map(|contract| { println!("{:?}", contract) })
      .count();
  }
}


fn main() {
  // something
}
