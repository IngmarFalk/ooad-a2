use super::app::App;
use crate::{
    models::domain::{contract::Contract, item::Item, system::LendingSystem, FromMap, ToMap},
    types::{Model, View},
    views::{
        contract_view::{ContractOption, ContractView},
        item_view::{CliItemView, ItemView},
    },
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
    fn ret(&self, display: &str) -> M {
        self.view.wait(display);
        self.model.clone()
    }

    fn fetch_item(&self) -> Option<Item> {
        let item_view: CliItemView = CliItemView::new();
        let items = self.model.get_items();
        let item = item_view.select_item(items).clone();
        match item {
            Some(i) => {
                let out = Item::default().copy_with(i.to_map());
                Some(out)
            }
            None => None,
        }
    }

    fn fetch_contract<F>(&self, fun: F) -> M
    where
        F: Fn(&Contract),
    {
        let item = self.fetch_item();
        match item {
            Some(i) => {
                let contracts = i.get_history().iter().collect::<Vec<&Contract>>();
                let contract = self.view.select_contract(contracts);
                match contract {
                    Some(c) => {
                        fun(c);
                        self.view.wait("");
                    }
                    None => {
                        self.view.wait("Couldnt find contract.");
                    }
                };
                self.model.clone()
            }
            None => self.model.clone(),
        }
    }

    fn display_contract_simple(&self) -> M {
        self.fetch_contract(|c: &Contract| self.view.display_contract_simple(c))
    }

    fn display_contract_verbose(&self) -> M {
        self.fetch_contract(|c: &Contract| self.view.display_contract_verbose(c))
    }

    fn create_contract(&mut self) -> M {
        let item = self.fetch_item();
        match item {
            Some(i) => {
                let contract = self.view.get_contract_info();
                let mut temp = i.clone();
                match temp.add_contract(contract) {
                    Ok(_) => {
                        match self.model.update_item(&i, &temp) {
                            Ok(_) => {}
                            Err(_) => todo!(),
                        }
                        self.ret("Successfully created contract.")
                    }
                    Err(_) => self.ret("Contract Already Exists."),
                }
            }
            None => self.ret("Couldnt retrieve item."),
        }
    }

    fn edit_contract(&mut self) -> M {
        let item = self.fetch_item();
        match item {
            Some(i) => {
                let mut history = i.get_history().clone();
                let contract = self
                    .view
                    .select_contract(history.iter().collect::<Vec<&Contract>>());
                match contract {
                    Some(c) => {
                        let new_contract_info = self.view.edit_contract(c);
                        match new_contract_info {
                            Some(new_contract) => {
                                let idx = history.index_of(&new_contract).unwrap();
                                history.set(idx, &new_contract);
                                self.view.wait("Updated contract successfully.")
                            }
                            None => {
                                self.view.wait("Couldnt edit contract.");
                            }
                        };

                        self.model.clone()
                    }
                    None => self.ret("Couldnt retrieve Contract."),
                }
            }
            None => self.ret("Couldnt retrieve item."),
        }
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
