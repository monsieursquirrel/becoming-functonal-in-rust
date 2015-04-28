pub fn get_enabled_customer_field(&self, func: ConversionFunction) -> Vec<&str> {
  let mut outlist: Vec<&str> = vec!();
  for customer in self.all_customers.iter() {
    if customer.enabled {
      outlist.push(func(customer));
    }
  }
  outlist
}
