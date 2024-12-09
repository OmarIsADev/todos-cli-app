use std::fs;
use std::io::stdin;

fn main() {
    let file_exists: bool = fs::metadata("todo-app-db.txt").is_ok();
    let mut data = Vec::new();

    if file_exists {
        data = fs::read_to_string("todo-app-db.txt")
            .expect("unable to read file")
            .split("\\n")
            .map(|line: &str| line.to_string())
            .collect();
    }

    loop {
        println!("=======================");
        println!("| 1. Add new Todo     |");
        println!("| 2. List Todos       |");
        println!("| 3. Remove Todo      |");
        println!("| 4. Exit             |");
        println!("=======================");

        let mut option: String = String::new();

        stdin()
            .read_line(&mut option)
            .expect("unable to read input");

        match option.trim() {
            "1" => {
                println!("What would you like to add?");

                let mut todo: String = String::new();

                stdin().read_line(&mut todo).expect("unable to read input");

                data.push(todo.trim().to_string());
            }
            "2" => {
                if data.len() == 0 {
                    println!("You dont have todos and you would like to see them?");
                    continue;
                }

                for (i, todo) in data.iter().enumerate() {
                    println!("{}. {}", i + 1, todo);
                }
            }
            "3" => {
                if data.len() == 0 {
                    println!("Unfortunately, There are no todos to remove");
                    continue;
                }

                println!("Which todo would you like to remove?");

                let mut todo: String = String::new();

                stdin().read_line(&mut todo).expect("unable to read input");

                if todo.trim().parse::<usize>().unwrap() > data.len()
                    || todo.trim().parse::<usize>().is_err()
                {
                    println!("Invalid todo number");
                    continue;
                }

                println!(
                    "Removed todo: {}",
                    data[todo.trim().parse::<usize>().unwrap() - 1]
                );

                data.remove(todo.trim().parse::<usize>().unwrap() - 1);
            }
            "4" => break,
            _ => println!("Invalid option"),
        }
    }

    fs::write("todo-app-db.txt", data.join("\\n")).expect("unable to write file");
}
