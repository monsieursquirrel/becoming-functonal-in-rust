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

    // has an additional step of building a new AllCustomers object so it can
    // be assigned to a suitable variable by the caller
    pub fn set_contract_for_customer_list(self,
        ids: Vec<usize>, status: bool) -> Self {
        self.update_contract_for_customer_list(ids, &|contract| {
                Contract {
                    enabled: status,
                    .. contract
                }
            })
    }

    pub fn update_customer_for_id_list(self,
        ids: Vec<usize>, cls: &Fn(Customer) -> Customer) -> Self {

        let new_customers = self.all_customers.into_iter()
        .map(|customer| {
            if ids.contains(&customer.id) {
                cls(customer)
            }
            else {
                customer
            }
        });

        AllCustomers {
            all_customers: new_customers.collect()
        }
    }

    pub fn update_contact(self, customer_id: usize, contact_id: usize,
        cls: &Fn(Contact) -> Contact) -> Self {

        self.update_customer_for_id_list(vec![customer_id], &|customer| {
            Customer{
                contacts: customer.contacts.into_iter()
                .map(|contact| {
                    if contact.contact_id == contact_id {
                        cls(contact)
                    }
                    else {
                        contact
                    }
                })
                .collect(),
                .. customer
            }
        })
    }

    pub fn update_contract_for_customer_list(self,
        ids: Vec<usize>, cls: &Fn(Contract) -> Contract) -> Self {

        self.update_customer_for_id_list(ids, &|customer| {
            Customer{
                contract: cls(customer.contract),
                .. customer
            }
        })
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
