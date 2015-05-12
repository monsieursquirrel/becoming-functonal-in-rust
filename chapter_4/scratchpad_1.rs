// nearly everything so far

extern crate chrono;

use chrono::{Date, Local, Duration};

#[derive(Debug)]
pub struct Contact {
    contact_id: usize,
    first_name: String,
    last_name: String,
    email: String,
    enabled: bool
}

impl Contact {
    pub fn new(contact_id: usize, first_name: String, last_name: String,
        email: String, enabled: bool) -> Self
    {
        Contact {
            contact_id: contact_id,
            first_name: first_name,
            last_name: last_name,
            email: email,
            enabled: enabled
        }
    }

    pub fn send_email(&self, message: &str) {
        // email code here!
    }
}


#[derive(Debug)]
pub struct Contract {
  // Date<Local> is a date in the local timezone
  begin_date: Date<Local>,
  end_date: Date<Local>,
  enabled: bool,
}

impl Contract {
  pub fn new(begin_date: Date<Local>, end_date: Date<Local>, enabled: bool) -> Self {
    Contract {
      begin_date: begin_date,
      end_date: end_date,
      enabled: enabled,
    }
  }

  pub fn new_today() -> Self {
    Contract::new(Local::today(), Local::today() + Duration::days(2 * 365), true)
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
  contacts: Vec<Contact>
}

impl Customer {
  pub fn new() -> Self {
    Customer {
      id: 0,
      name: "".to_string(),
      address: "".to_string(),
      state: "".to_string(),
      primary_contact: "".to_string(),
      domain: "".to_string(),
      enabled: true,
      contract: Contract::new_today(),
      contacts: Vec::new()
    }
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

    pub fn print_set_contract_for_customer(self,
        ids: Vec<usize>, status: bool) -> Vec<Customer>{

        // Note: into_iter() not iter(). This turns the vec
        // into an iterator parmanently, allowing the values
        // to be moved out.
        self.all_customers.into_iter()
        .map(|customer| {
            if ids.contains(&customer.id) {
                Customer {
                    contract: Contract {
                        enabled: status,
                        .. customer.contract
                    },
                    .. customer
                }
            }
            else {
                customer
            }
        })
        .collect()
    }


    pub fn send_enabled_customers_emails(&self, message: &str) {
        self.all_customers.iter()
        .filter(|customer| {
            customer.enabled && customer.contract.enabled
        })
        .map(|customer| {
                customer.contacts.iter()
                .filter(|contact| {
                    contact.enabled
                })
                .map(|contact| {
                    contact.send_email(message)
                })
                .count(); // just need to call something to consume the iterator

            })
        .count();
    }

    pub fn each_enabled_contact<'a>(&'a self, cls: &Fn(&'a Contact)) {
        self.all_customers.iter()
        .filter(|customer| {
            customer.enabled && customer.contract.enabled
        })
        .map(|customer| {
                customer.contacts.iter()
                .filter(|contact| {
                    contact.enabled
                })
                .map(cls)
                .count(); // just need to call something to consume the iterator

            })
        .count();
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
}


fn main() {
  // something
}