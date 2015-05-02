
// First change from the book - no statics!*
//
// *technically, statics do exist in Rust. However, the compiler has to be very
// strict about not modifying them as they are shared data.
pub struct AllCustomers {
  pub all_customers: Vec<Customer>,
  pub id: usize,
}

impl AllCustomers {
  pub fn new() -> Self {
    AllCustomers {
      all_customers: vec!(),
      id: 0
    }
  }

  pub fn get_enabled_customer_names(&self) -> Vec<&str> {
    let mut outlist: Vec<&str> = vec!();
    for customer in self.all_customers.iter() {
      if customer.enabled {
        outlist.push(&customer.name);
      }
    }
    outlist
  }

  pub fn get_enabled_customer_addresses(&self) -> Vec<&str> {
    let mut outlist: Vec<&str> = vec!();
    for customer in self.all_customers.iter() {
      if customer.enabled {
        outlist.push(&customer.address);
      }
    }
    outlist
  }

  pub fn get_enabled_customer_states(&self) -> Vec<&str> {
    let mut outlist: Vec<&str> = vec!();
    for customer in self.all_customers.iter() {
      if customer.enabled {
        outlist.push(&customer.state);
      }
    }
    outlist
  }

  pub fn get_enabled_customer_primary_contacts(&self) -> Vec<&str> {
    let mut outlist: Vec<&str> = vec!();
    for customer in self.all_customers.iter() {
      if customer.enabled {
        outlist.push(&customer.primary_contact);
      }
    }
    outlist
  }

  pub fn get_enabled_customer_domains(&self) -> Vec<&str> {
    let mut outlist: Vec<&str> = vec!();
    for customer in self.all_customers.iter() {
      if customer.enabled {
        outlist.push(&customer.domain);
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
