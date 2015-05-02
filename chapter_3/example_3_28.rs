// same as 2.19

pub fn get_disabled_customer_names(&self) -> Vec<&str> {
  self.all_customers.iter()
    .filter(|customer| { !customer.enabled })
    .map(|customer| { &customer.name[..] })
    .collect()
}
