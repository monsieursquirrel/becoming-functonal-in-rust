pub fn get_disabled_field<'a, B, F>(&'a self, func: F) -> Vec<B>
  where F: Fn(&'a Customer) -> B {

  let mut outlist: Vec<B> = vec!();
  for customer in self.all_customers.iter() {
    if !customer.enabled {
      outlist.push(func(customer));
    }
  }
  outlist
}
