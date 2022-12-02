fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

pub fn main(input: &str) -> String {
    let result = input
    println!("{:?}", type_of(&result));
    result.to_string()
}
