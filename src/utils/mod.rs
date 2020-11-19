// Copyright (c) 2020 sss developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `sss` testing utilities

use crate::error::Result;
use rand::{rngs::ThreadRng, seq::IteratorRandom};
use std::{collections::HashMap, hash::Hash};

#[allow(dead_code)]
crate fn print_shares(map: &HashMap<u8, Vec<u8>>) {
    for (k, v) in map {
        println!("Key: {}, Value: {:?}", k, v);
    }
}

crate fn remove_random_entry<T, U>(rng: &mut ThreadRng, map: &mut HashMap<T, U>)
where
    T: Clone + Hash + Eq,
{
    let _ = choose_idx(rng, map).and_then(|idx| map.remove(&idx));
}

fn choose_idx<T, U>(rng: &mut ThreadRng, map: &HashMap<T, U>) -> Option<T>
where
    T: Clone,
{
    map.clone().keys().choose(rng).cloned()
}

crate fn check_err_result<T>(result: Result<T>, expected: &str) -> Result<()> {
    assert!(result.is_err());
    match result {
        Ok(_) => Err("invalid error result".into()),
        Err(e) => {
            assert_eq!(format!("{}", e), expected);
            Ok(())
        }
    }
}