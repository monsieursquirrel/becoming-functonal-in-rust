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
    self.get_enabled_customer_field(customer_name)
  }

  pub fn get_enabled_customer_addresses(&self) -> Vec<&str> {
    self.get_enabled_customer_field(customer_address)
  }

  pub fn get_enabled_customer_states(&self) -> Vec<&str> {
    self.get_enabled_customer_field(customer_state)
  }

  pub fn get_enabled_customer_primary_contacts(&self) -> Vec<&str> {
    self.get_enabled_customer_field(customer_primary_contact)
  }

  pub fn get_enabled_customer_domains(&self) -> Vec<&str> {
    self.get_enabled_customer_field(customer_domain)
  }

  pub fn get_enabled_customers(&self) -> Vec<&Customer> {
    self.get_enabled_customer_field(customer_as_customer)
  }

  pub fn get_enabled_customer_field<B: ?Sized>(&self, func: fn(&Customer) -> &B) -> Vec<&B> {
    let mut outlist: Vec<&B> = vec!();
    for customer in self.all_customers.iter() {
      if customer.enabled {
        outlist.push(func(customer));
      }
    }
    outlist
  }
}

fn customer_name(customer: &Customer) -> &str {
  &customer.name
}

fn customer_address(customer: &Customer) -> &str {
  &customer.address
}

fn customer_state(customer: &Customer) -> &str {
  &customer.state
}

fn customer_primary_contact(customer: &Customer) -> &str {
  &customer.primary_contact
}

fn customer_domain(customer: &Customer) -> &str {
  &customer.domain
}

fn customer_as_customer(customer: &Customer) -> &Customer {
  customer
}

pub struct Customer {
  id: usize,
  name: String,
  address: String,
  state: String,
  primary_contact: String,
  domain: String,
  enabled: bool
}


fn main() {
  // something
}
