use super::app::App;
use crate::{
    models::system::LendingSystem,
    types::{Model, View},
    views::contract_view::{ContractOption, ContractView},
};
use shared::controller;

#[derive(Debug, Clone)]
#[controller(ContractView)]
pub struct ContractController<M, V>
where
    M: Model + LendingSystem,
    V: View + ContractView,
{
    model: M,
    view: V,
}

impl<M, V> ContractController<M, V>
where
    M: Model + LendingSystem + Clone,
    V: View + ContractView,
{
    fn display_contract_simple(&self) {
        // let contracts = self.model.
    }

    fn display_contract_verbose(&self) {
        todo!()
    }

    fn create_contract(&mut self) {
        todo!()
    }

    fn edit_contract(&mut self) {
        todo!()
    }
}

impl<M, V> App<M> for ContractController<M, V>
where
    M: Model + LendingSystem + Clone,
    V: View + ContractView,
{
    fn run(&mut self, sys: M) -> M {
        let choice = self.view.contract_menu();
        match choice {
            ContractOption::DisplayContractSimple => self.display_contract_simple(),
            ContractOption::DisplayContractVerbose => self.display_contract_verbose(),
            ContractOption::CreateContract => self.create_contract(),
            ContractOption::EditContract => self.edit_contract(),
            ContractOption::Quit => std::process::exit(0),
            _ => sys,
        }
    }
}
