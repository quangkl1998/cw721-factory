use cosmwasm_schema::write_api;

use cw721_factory::msg::{FactoryExecuteMsg, FactoryInstantiateMsg, MigrateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: FactoryInstantiateMsg,
        migrate: MigrateMsg,
        execute: FactoryExecuteMsg,
        query: QueryMsg,
    }
}
