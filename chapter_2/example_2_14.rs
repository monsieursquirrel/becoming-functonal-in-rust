// A couple of issues to note on this one:
// - Closures are memory objects like most other data; when using dynanoc
//   dispatch, they have to be borrowed.
// - Type inference was guessing that the closures wanted to return references
//   to String when the functions are returning a vector of string slices
//   (&str). Fixed by explicitly slicing the whole string using [..].
// - The lifetime of the parameter passed to the closure has to be tied
//   to the lifetime of self using a lifetime parameter ('a), this could not
//   be inferred.


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
    self.get_enabled_customer_field(&|customer| { &customer.name[..] })
  }

  pub fn get_enabled_customer_addresses(&self) -> Vec<&str> {
    self.get_enabled_customer_field(&|customer| { &customer.address[..] })
  }

  pub fn get_enabled_customer_states(&self) -> Vec<&str> {
    self.get_enabled_customer_field(&|customer| { &customer.state[..] })
  }

  pub fn get_enabled_customer_primary_contacts(&self) -> Vec<&str> {
    self.get_enabled_customer_field(&|customer| { &customer.primary_contact[..] })
  }

  pub fn get_enabled_customer_domains(&self) -> Vec<&str> {
    self.get_enabled_customer_field(&|customer| { &customer.domain[..] })
  }

  pub fn get_enabled_customers(&self) -> Vec<&Customer> {
    self.get_enabled_customer_field(&|customer| { customer })
  }

  pub fn get_enabled_customer_field<'a, B>(&'a self, func: &Fn(&'a Customer) -> B) -> Vec<B> {

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
