#[macro_export]
macro_rules! odata_query {
    (
        $($query_item:expr),*
    ) => {
        {
            let mut v = Vec::new();
            $(
                v.push(format!("{}", $query_item));
            )*
            v.join("")
        }
    };
}
