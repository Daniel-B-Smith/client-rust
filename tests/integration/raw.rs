// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use crate::{arb_batch, integration::pd_addr};
use futures::executor::block_on;
use proptest::{arbitrary::any, proptest};
use tikv_client::{raw::Client, Config, KvPair, Value};

proptest! {
    /// Test single point (put, get, delete) operations on keys.
    #[test]
    fn point(
        pair in any::<KvPair>(),
    ) {
        let client = block_on(Client::connect(Config::new(pd_addr()))).unwrap();

        block_on(
            client.put(pair.key().clone(), pair.value().clone())
        ).unwrap();

        let out_value = block_on(
            client.get(pair.key().clone())
        ).unwrap();
        assert_eq!(Some(Value::from(pair.value().clone())), out_value);

        block_on(
            client.delete(pair.key().clone())
        ).unwrap();
    }
}

proptest! {
    /// Test batch (put, get, delete) operations on keys.
    #[test]
    fn batch(
        kvs in arb_batch(any::<KvPair>(), None),
    ) {
        let client = block_on(Client::connect(Config::new(pd_addr()))).unwrap();
        let keys = kvs.iter().map(|kv| kv.key()).cloned().collect::<Vec<_>>();

        block_on(
            client.batch_put(kvs.clone())
        ).unwrap();

        let out_value = block_on(
            client.batch_get(keys.clone())
        ).unwrap();
        assert_eq!(kvs, out_value);

        block_on(
            client.batch_delete(keys.clone())
        ).unwrap();
    }
}
