pub fn set_contract_enabled_for_customer(inlist: &mut Vec<Customer>, customer_id: usize) {
  for customer in get_customer_by_id(inlist, customer_id) {
    customer.contract.enabled = true;
  }
}
