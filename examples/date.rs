use inquire::DateSelect;

fn main() {
    let date = DateSelect::new("Simple input").prompt().unwrap();

    println!("{}", date);

    let date = DateSelect::new("Validated input")
        .with_validator(|d| {
            let now = chrono::Utc::now().naive_utc().date();

            if d.ge(&now) {
                Err("Date must be in the past".into())
            } else {
                Ok(())
            }
        })
        .prompt()
        .unwrap();

    println!("{}", date);
}
