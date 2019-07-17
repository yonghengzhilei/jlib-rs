extern crate jlib;

use jlib::misc::config::*;
use jlib::api::query::ledger_closed::*;
use jlib::message::ledger_closed::{LedgerClosedResponse};

use std::rc::Rc;

fn main() {
    let config: Box<Rc<Config>> = Config::new(TEST1, true);
    println!("config : {:?}", config);

    let _c = LedgerClosed::new().request_ledger_closed(config.clone(), |x| match x {
        Ok(response) => {
            let res: LedgerClosedResponse = response;
            println!("----------------------------------------------------------------------------------");
            println!("最新账本信息 : ");
            println!("-- 账本Hash : {}", &res.ledger_hash);
            println!("-- 账本Index: {}", &res.ledger_index);
            println!("----------------------------------------------------------------------------------");
        }

        Err(_) => {
            panic!("Error Message.");
        }
    });
}