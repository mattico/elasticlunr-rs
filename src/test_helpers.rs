
// From `maplit`, modified to use FnvHasher
macro_rules! hashmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { hashmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = hashmap!(@count $($key),*);
            let mut _map = ::fnv::FnvHashMap::with_capacity_and_hasher(10, Default::default());
            $(
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}

// From `maplit`
macro_rules! btreemap {
    // trailing comma case
    ($($key:expr => $value:expr,)+) => (btreemap!($($key => $value),+));

    ( $($key:expr => $value:expr),* ) => {
        {
            let mut _map = ::std::collections::BTreeMap::new();
            $(
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}
