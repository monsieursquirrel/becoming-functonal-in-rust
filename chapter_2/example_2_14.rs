// A couple of issues to note on this one:
// - Rust treats each closure (not really distinct from lambdas) as a different
//   type, this means that the function parameter on get_enabled_customer_field
//   has to be made generic. The knock on effect of this is that the return type
//   doesn't have to be a reference as the lifetime is now determines when the
//   generic is instantiated.
// - Type inference was guessing that the closures wanted to return references
//   to String when the functions are returning a vector of string slices
//   (&str). Fixed by explicitly slicing the whole string using [..].
// - get_enabled_customer_field has an example of using a where clause, this is
//   an alternate way to layout generic bounds which sometimes looks better.


pub type ConversionFunction = fn(&Customer) -> String;

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
    self.get_enabled_customer_field(|customer| { &customer.name[..] })
  }

  pub fn get_enabled_customer_addresses(&self) -> Vec<&str> {
    self.get_enabled_customer_field(|customer| { &customer.address[..] })
  }

  pub fn get_enabled_customer_states(&self) -> Vec<&str> {
    self.get_enabled_customer_field(|customer| { &customer.state[..] })
  }

  pub fn get_enabled_customer_primary_contacts(&self) -> Vec<&str> {
    self.get_enabled_customer_field(|customer| { &customer.primary_contact[..] })
  }

  pub fn get_enabled_customer_domains(&self) -> Vec<&str> {
    self.get_enabled_customer_field(|customer| { &customer.domain[..] })
  }

  pub fn get_enabled_customers(&self) -> Vec<&Customer> {
    self.get_enabled_customer_field(|customer| { customer })
  }

  pub fn get_enabled_customer_field<'a, B, F>(&'a self, func: F) -> Vec<B>
    where F: Fn(&'a Customer) -> B {

    let mut outlist: Vec<B> = vec!();
    for customer in self.all_customers.iter() {
      if customer.enabled {
        outlist.push(func(customer));
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
