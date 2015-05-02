pub fn set_contract_enabled_for_customer(inlist: &mut Vec<Customer>, customer_id: usize) {
  for_each(get_customer_by_id(inlist, customer_id), |customer| {
    customer.contract.enabled = true;
  });
}
