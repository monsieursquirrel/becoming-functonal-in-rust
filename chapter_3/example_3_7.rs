pub fn set_contract_enabled_for_customer(&mut self, customer_id: usize) {
  for customer in self.get_customer_by_id(customer_id) {
    customer.contract.enabled = true;
  }
}
