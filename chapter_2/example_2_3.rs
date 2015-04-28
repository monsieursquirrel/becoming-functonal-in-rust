pub fn get_enabled_customer_field(&self) -> Vec<&str> {
  let mut outlist: Vec<&str> = vec!();
  for customer in self.all_customers.iter() {
    if customer.enabled {
      // placeholder
    }
  }
  outlist
}
