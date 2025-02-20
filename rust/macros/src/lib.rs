#[macro_export]
macro_rules! hashmap {
    () => {
        {
            ::std::collections::HashMap::new()
        }
    };
    ($($key:expr => $value:expr),+ $(,)?) => {
        {
            let mut h = ::std::collections::HashMap::new();
            $(
                h.insert($key, $value);
            )*
            h
        }
    };
}
