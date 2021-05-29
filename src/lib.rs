#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod request;

use liber::Lib;
use request::load as load_request;
use std::rc::Rc;
use structures::structs::{DefaultTypes, Env, Function};

/// Helper trait
/// That converts <Fn>s to Function structures
trait AsFuncObj {
    fn as_obj(&'static self) -> Function;
}

impl<T> AsFuncObj for T
where
    T: Fn(&mut Env, Vec<DefaultTypes>) -> Vec<DefaultTypes>,
{
    fn as_obj(&'static self) -> Function {
        Function::new(Rc::new(self))
    }
}

pub fn loader(e: &mut Env) {
    load_request().load(e);
}
