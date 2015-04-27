
// First change from the book - no statics!
// Rust restricts their usage for memory safety reasons.
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
    let mut outlist: Vec<String> = vec!();
    for customer in self.all_customers.iter() {
      if customer.enabled {
        outlist.push(customer.name.clone());
      }
    }
    outlist
  }

  pub fn get_enabled_customer_states(&self) -> Vec<String> {
    let mut outlist: Vec<String> = vec!();
    for customer in self.all_customers.iter() {
      if customer.enabled {
        outlist.push(customer.state.clone());
      }
    }
    outlist
  }

  pub fn get_enabled_customer_primary_contacts(&self) -> Vec<String> {
    let mut outlist: Vec<String> = vec!();
    for customer in self.all_customers.iter() {
      if customer.enabled {
        outlist.push(customer.primary_contact.clone());
      }
    }
    outlist
  }

  pub fn get_enabled_customer_domains(&self) -> Vec<String> {
    let mut outlist: Vec<String> = vec!();
    for customer in self.all_customers.iter() {
      if customer.enabled {
        outlist.push(customer.domain.clone());
      }
    }
    outlist
  }
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
