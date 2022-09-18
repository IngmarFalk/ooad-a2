use crate::{models::domain::contract::Contract, types::View};

use super::console::Console;

pub trait ContractDisplay {
    fn display_contract_simple(&self, contract: Contract);
    fn display_contract_verbose(&self, contract: Contract);
}

pub struct ContractView {
    console: Console,
}

impl ContractDisplay for ContractView {
    fn display_contract_simple(&self, contract: Contract) {
        todo!()
    }

    fn display_contract_verbose(&self, contract: Contract) {
        todo!()
    }
}

impl View for ContractView {}
