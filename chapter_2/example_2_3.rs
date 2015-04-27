pub fn get_enabled_customer_field(&self) -> Vec<String> {
  let mut outlist: Vec<String> = vec!();
  for customer in self.all_customers.iter() {
    if customer.enabled {
      // placeholder
    }
  }
  outlist
}
