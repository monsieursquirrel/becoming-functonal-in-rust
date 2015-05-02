pub fn print_set_contract_for_customer(&mut self,
                                       customer_id: usize,
                                       status: bool) {
  self.all_customers.iter_mut().filter(|customer| {
    customer.id == customer_id
  }).map(|customer| {
    customer.contract.set_enabled(status)
  }).map(|contract| {
    println!("{:?}", contract)
  }).count();
}
