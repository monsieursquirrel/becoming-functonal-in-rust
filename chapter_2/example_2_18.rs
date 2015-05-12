pub fn get_field<'a, B>(&'a self,
    test: Fn(&'a Customer) -> bool,
    func: &Fn(&'a Customer) -> B)
    -> Vec<B>
{

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
  self.get_field(&Self::disabled_customer, &|customer| { &customer.name[..] })
}
