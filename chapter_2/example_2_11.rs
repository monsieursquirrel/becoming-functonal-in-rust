// Example 2.6 in context

// A trait would be a more direct translation of an interface.
// However it would be very heavy in a situation where no actual struct data is
// required.
pub type ConversionFunction = fn(&Customer) -> String;

pub struct AllCustomers {
  pub all_customers: Vec<Customer>,
  pub id: usize,
}

impl AllCustomers {
  pub fn new() -> AllCustomers {
    AllCustomers {
      all_customers: vec!(),
      id: 0
    }
  }

  pub fn get_enabled_customer_names(&self) -> Vec<String> {
    self.get_enabled_customer_field(customer_name)
  }

  pub fn get_enabled_customer_addresses(&self) -> Vec<String> {
    self.get_enabled_customer_field(customer_address)
  }

  pub fn get_enabled_customer_states(&self) -> Vec<String> {
    self.get_enabled_customer_field(customer_state)
  }

  pub fn get_enabled_customer_primary_contacts(&self) -> Vec<String> {
    self.get_enabled_customer_field(customer_primary_contact)
  }

  pub fn get_enabled_customer_domains(&self) -> Vec<String> {
    self.get_enabled_customer_field(customer_domain)
  }

  pub fn get_enabled_customer_field<B>(&self, func: fn(&Customer) -> B) -> Vec<B> {
    let mut outlist: Vec<B> = vec!();
    for customer in self.all_customers.iter() {
      if customer.enabled {
        outlist.push(func(customer));
      }
    }
    outlist
  }
}

fn customer_name(customer: &Customer) -> String {
  customer.name.clone()
}

fn customer_address(customer: &Customer) -> String {
  customer.address.clone()
}

fn customer_state(customer: &Customer) -> String {
  customer.state.clone()
}

fn customer_primary_contact(customer: &Customer) -> String {
  customer.primary_contact.clone()
}

fn customer_domain(customer: &Customer) -> String {
  customer.domain.clone()
}

pub struct Customer {
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
