// Typing out all those string comparisons made me feel dirty so here's
// a more rust-idiomatic version of example 2.4 using an enum and a match.

pub struct AllCustomers {
  pub all_customers: Vec<Customer>,
  pub id: usize,
}


pub enum FieldName {
  Name,
  State,
  PrimaryContact,
  Domain,
  Address
}

impl AllCustomers {
  pub fn new() -> AllCustomers {
    AllCustomers {
      all_customers: vec!(),
      id: 0
    }
  }

  pub fn get_enabled_customer_field(&self, field: FieldName) -> Vec<String> {
    let mut outlist: Vec<String> = vec!();
    for customer in self.all_customers.iter() {
      if customer.enabled {
        match field {
          FieldName::Name => { outlist.push(customer.name.clone()); },
          FieldName::State => { outlist.push(customer.state.clone()); },
          FieldName::PrimaryContact => { outlist.push(customer.primary_contact.clone()); },
          FieldName::Domain => { outlist.push(customer.domain.clone()); },
          FieldName::Address => { outlist.push(customer.address.clone()); },
        }
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
