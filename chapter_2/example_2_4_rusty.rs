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
  pub fn new() -> Self {
    AllCustomers {
      all_customers: vec!(),
      id: 0
    }
  }

  pub fn get_enabled_customer_field(&self, field: FieldName) -> Vec<&str> {
    let mut outlist: Vec<&str> = vec!();
    for customer in self.all_customers.iter() {
      if customer.enabled {
        match field {
          FieldName::Name => { outlist.push(&customer.name); },
          FieldName::State => { outlist.push(&customer.state); },
          FieldName::PrimaryContact => { outlist.push(&customer.primary_contact); },
          FieldName::Domain => { outlist.push(&customer.domain); },
          FieldName::Address => { outlist.push(&customer.address); },
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
