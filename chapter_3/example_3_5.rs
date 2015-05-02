pub fn get_field<'a, B, F>(&'a mut self,
                           test: fn(&Customer) -> bool,
                           func: F) -> Vec<B>
  where F: Fn(&'a Customer) -> B {

  let mut outlist: Vec<B> = vec!();
  for customer in self.filter(test) {
    outlist.push(func(customer));
  }
  outlist
}
