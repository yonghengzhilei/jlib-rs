//
// 获得账号关系
//
extern crate ws;
use ws::{connect, CloseCode};
use std::rc::Rc;
use std::cell::Cell;
use serde_json::{Value};

use crate::misc::config::*;
use crate::message::query::relations::*;
use crate::message::common::command_trait::CommandConversion;
use crate::base::misc::util::downcast_to_string;

pub trait RelationsI {
    fn request_account_relations<F>(&self, config: Box<Rc<Config>>, account: String, relation_type: Option<String>, op: F)
    where F: Fn(Result<RequestAccountRelationsResponse, RelationsSideKick>);
}

pub struct Relations {}
impl Relations {
    pub fn new() -> Self {
        Relations {
        }
    }
}

impl RelationsI for Relations {
    fn request_account_relations<F>(&self, config: Box<Rc<Config>>, account: String, relation_type: Option<String>, op: F)
    where F: Fn(Result<RequestAccountRelationsResponse, RelationsSideKick>) {

        let info = Rc::new(Cell::new("".to_string()));

        let account_rc = Rc::new(Cell::new(account));
        let relation_type_rc = Rc::new(Cell::new(relation_type));
        connect(config.addr, |out| {
            let copy = info.clone();

            let account = account_rc.clone();
            let relation_type = relation_type_rc.clone();
            if let Ok(command) = RequestAccountRelationsCommand::with_params(account.take(), relation_type.take()).to_string() {
                out.send(command).unwrap();
            }

            move |msg: ws::Message| {

                let c = msg.as_text()?;
                copy.set(c.to_string());

                out.close(CloseCode::Normal)
            }

        }).unwrap();

        let resp = downcast_to_string(info);
        if let Ok(x) = serde_json::from_str(&resp) as Result<Value, serde_json::error::Error> {
            if let Some(status) = x["status"].as_str() {
                if status == "success" {
                    let x: String = x["result"].to_string();
                    if let Ok(v) = serde_json::from_str(&x) as Result<RequestAccountRelationsResponse, serde_json::error::Error> {
                        op(Ok(v))
                    }
                } else {
                    if let Ok(v) = serde_json::from_str(&x.to_string()) as Result<RelationsSideKick, serde_json::error::Error> {
                        op(Err(v))
                    }
                }
            }
        }
    }
}
