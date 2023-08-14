fn try_greet(option_name: Option<&str>) {
    match option_name {
        Some(name) => println!("Name is {}", name),
        None => println!("No name provided"),
    }
}

fn main() {
    let option_name: Option<String> = Some("Alice".to_owned());
    try_greet(option_name.as_deref());
    println!("{:?}", option_name);
}