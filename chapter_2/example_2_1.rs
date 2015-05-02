
pub struct Customer {
  id: usize,
  name: String,
  address: String,
  state: String,
  primary_contact: String,
  domain: String,
  enabled: bool
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
    }
  }
}

// First change from the book - no statics!*
// Instead, I'm using this struct to store the customer list. This can be owned
// by whatever bit of code needs to work on the customer list.
//
// *technically, statics do exist in Rust. However, the compiler has to be very
// strict about not modifying them as they are shared data.

pub struct AllCustomers {
  all_customers: Vec<Customer>,
}

impl AllCustomers {
  pub fn new() -> Self {
    AllCustomers {
      all_customers: vec!(),
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

fn main() {
  // something
}
