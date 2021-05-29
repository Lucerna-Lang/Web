//! Library for functions that only work with numbers
//! Currently: multiply, larger, abs, neg

use crate::{AsFuncObj, Lib};
use reqwest::blocking;
use structures::structs::{DefaultTypes, Env, Table};

// TODO: Add headers
pub fn get(_: &mut Env, mut v: Vec<DefaultTypes>) -> Vec<DefaultTypes> {
    let d = v.remove(0);
    if let DefaultTypes::Str(url) = d {
        let resp = blocking::get(url).expect("Error sending request");
        let mut t = Table::new();
        t.set(
            "status".parse().unwrap(),
            DefaultTypes::Int(i32::from(resp.status().as_u16())),
        );
        t.set(
            "body".parse().unwrap(),
            DefaultTypes::Str(resp.text().unwrap()),
        );
        vec![DefaultTypes::Table(t)]
    } else {
        v
    }
}

pub fn load() -> Lib {
    let mut s = Lib::new("request");
    s.add("get", get.as_obj());
    s
}
