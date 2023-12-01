// macro that gets the path to the input file which is in the src/dayX. This is used in every day
#[macro_export]
macro_rules! day_input_path {
    () => {{
        use std::path::Path;
        Path::new(file!()).parent().unwrap().join("input.txt")
    }};
}

#[macro_export]
macro_rules! day_input_to_string {
    () => {{
        use crate::day_input_path;
        use std::fs;
        fs::read_to_string(day_input_path!()).unwrap()
    }};
}

#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => {{
        use std::collections::HashMap;

        let mut map = HashMap::new();
        $(map.insert($key, $value);)*
        map
    }};
}
