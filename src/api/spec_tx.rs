//
// 查询某一交易具体信息
//
extern crate ws;
use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::misc::config::*;
use crate::commands::spec_tx::*;
use crate::commands::command_trait::CommandConversion;
use crate::base::util::downcast_to_string;

pub trait SpecTxI {
    fn request_tx<F>(&self, config: Box<Rc<Config>>, hash: String,  op: F) 
    where F: Fn(Result<RequestTxResponse, serde_json::error::Error>);
}

pub struct SpecTx {}
impl SpecTx {
    pub fn new() -> Self {
        SpecTx {
        }
    }
}

impl SpecTxI for SpecTx {
    fn request_tx<F>(&self, config: Box<Rc<Config>>, hash: String,  op: F) 
    where F: Fn(Result<RequestTxResponse, serde_json::error::Error>) {

        let info = Rc::new(Cell::new("".to_string()));

        let hash_rc = Rc::new(Cell::new(hash));
        
        connect(config.addr, |out| { 
            let copy = info.clone();

            let hash = hash_rc.clone();

            if let Ok(command) = RequestTxCommand::with_params(hash.take()).to_string() {
                out.send(command).unwrap();
            }

            //返回一个Handler类型(trait)，等待epoll调用。
            move |msg: ws::Message| {
                let c = msg.as_text()?;
                copy.set(c.to_string());
                
                out.close(CloseCode::Normal) 
            }
        
        }).unwrap();
        
        let resp = downcast_to_string(info);
        if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
            let x: String = x["result"].to_string();
            if let Ok(v) = serde_json::from_str(&x) as Result<RequestTxResponse, serde_json::error::Error> {
                op(Ok(v))
            }
        }         
    }
}