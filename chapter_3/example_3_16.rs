pub fn set_contract_for_customer(inlist: &mut Vec<Customer>,
                                 customer_id: usize,
                                 status: bool) {
  for_each(get_customer_by_id(inlist, customer_id), |customer| {
    customer.contract.enabled = true;
  });
}
