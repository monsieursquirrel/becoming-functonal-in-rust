pub fn print_set_contract_for_customer(inlist: &mut Vec<Customer>,
                                       customer_id: usize,
                                       status: bool) {
  for_each(Self::set_contract_for_customer(inlist, customer_id, status),
          |contract| { println!("{:?}", contract); })
}
