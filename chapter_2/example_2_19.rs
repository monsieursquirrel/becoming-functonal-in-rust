// In Rust, list operations such as filtering and mapping are usually performed
// on iterators. These behave lazily unless you loop over them or collect()
// them.

pub fn get_enabled_customer_names(&self) -> Vec<&str> {
  self.all_customers.iter()
    .filter(|customer| { customer.enabled })
    .map(|customer| { &customer.name[..] })
    .collect()
}

pub fn get_disabled_customer_names(&self) -> Vec<&str> {
  self.all_customers.iter()
    .filter(|customer| { !customer.enabled })
    .map(|customer| { &customer.name[..] })
    .collect()
}
