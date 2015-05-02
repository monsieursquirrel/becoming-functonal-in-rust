pub fn set_contract_for_customer(inlist: &mut Vec<Customer>,
                                 customer_id: usize,
                                 status: bool) {
  map(get_customer_by_id(inlist, customer_id), |customer| {
    customer.contract.set_enabled(status)
  });
}
