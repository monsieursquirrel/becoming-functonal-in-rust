pub fn get_enabled_customer_field<B: ?Sized>(&self, func: fn(&Customer) -> &B) -> Vec<&B> {
  let mut outlist: Vec<&B> = vec!();
  for customer in self.all_customers.iter() {
    if customer.enabled {
      outlist.push(func(customer));
    }
  }
  outlist
}
