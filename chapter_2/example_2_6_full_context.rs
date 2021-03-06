// Example 2.6 in context

// A trait would be a more direct translation of an interface.
// However it would be very heavy in a situation where no actual struct data is
// required.
pub type ConversionFunction = fn(&Customer) -> &str;

pub struct AllCustomers {
  pub all_customers: Vec<Customer>,
}

impl AllCustomers {
  pub fn new() -> Self {
    AllCustomers {
      all_customers: vec!(),
    }
  }

  pub fn get_enabled_customer_field(&self, func: ConversionFunction) -> Vec<&str> {
    let mut outlist: Vec<&str> = vec!();
    for customer in self.all_customers.iter() {
      if customer.enabled {
        outlist.push(func(customer));
      }
    }
    outlist
  }
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
