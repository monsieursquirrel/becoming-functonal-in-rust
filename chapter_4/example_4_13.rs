pub fn set_contract_for_customer_list(self,
    ids: Vec<usize>, status: bool) -> Self {
    self.update_contract_for_customer_list(ids, &|contract| {
            Contract {
                enabled: status,
                .. contract
            }
        })
}
