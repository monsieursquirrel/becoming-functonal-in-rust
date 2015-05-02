// This uses the count() function to consume the iterator, otherwise we'd just
// generate a lazy iterator and not use it.

pub fn set_contract_for_customer(&mut self,
                                 customer_id: usize,
                                 status: bool) {
  self.all_customers.iter_mut()
    .filter(|customer| { customer.id == customer_id })
    .map(|customer| { customer.contract.set_enabled(status) })
    .map(|contract| { println!("{:?}", contract) })
    .count();
}
