use std::{path::PathBuf, str::FromStr};

use crate::do_procedure;

#[test]
fn assertion_check() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    let _cal = do_procedure(vec![PathBuf::from_str("qld.ics").unwrap()], 365);
}
