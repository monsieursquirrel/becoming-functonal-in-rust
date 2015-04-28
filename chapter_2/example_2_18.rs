pub fn get_field<'a, B, F, T>(&'a self, test: T, func: F) -> Vec<B>
  where F: Fn(&'a Customer) -> B, T: Fn(&'a Customer) -> bool {

  let mut outlist: Vec<B> = vec!();
  for customer in self.all_customers.iter() {
    if test(customer) {
      outlist.push(func(customer));
    }
  }
  outlist
}

pub fn enabled_customer(customer: &Customer) -> bool {
  customer.enabled
}


pub fn disabled_customer(customer: &Customer) -> bool {
  !customer.enabled
}

pub fn get_disabled_customer_names(&self) -> Vec<&str> {
  self.get_field(Self::disabled_customer, |customer| { &customer.name[..] })
}
