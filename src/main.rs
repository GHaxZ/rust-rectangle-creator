// TODO: add option to change the size of already existing rectangles

use std::io;
use std::collections::HashMap;

struct Rectangle{
    height: i32,
    width: i32,
}

impl Rectangle {
    fn draw(&self) {
        for _ in 0..self.height {
            for _ in 0..self.width {
                print!("#")
            }
            println!();
        }
    }
}

fn main() {
    let mut rects = HashMap::<String, Rectangle>::new();

    loop {
        println!("Enter a command (new, show, draw, change): ");

        let command = str_input().to_lowercase().replace(" ", "");

        if command == "new" {
            println!("Enter the height of the rectangle: ");
            let height = int_input();

            println!("Enter the width of the rectangle: ");
            let width = int_input();

            let name = loop {
                println!("\nGive your rectangle a name: ");

                let name = str_input();

                if rects.contains_key(&name) {
                    println!("{} is already used.", name);
                } else {
                    break name
                }
            };


            let rectangle = Rectangle {
                height,
                width,
            };

            println!("You successfully created a new rectangle called {}.", name);

            rects.insert(name, rectangle);
        } else if command == "show" {
            print_rects(&rects);
        } else if command == "draw" {
            loop {
                println!("\nThese are your current rectangles: ");
                print_rects(&rects);

                println!("\nPlease enter the name of the rectangle you want to print: ");
                let input = str_input();

                if rects.contains_key(&input) {
                    if let Some(rect) = rects.get(&input) {
                        rect.draw();
                        break;
                    }
                }

                println!("No rectangle found with the name {}", input);
            }

        }
    }
}

fn print_rects(rects: &HashMap<String, Rectangle>) {
    let mut iteration = 1;
    for rect in rects.iter() {
        if iteration > 3 {
            println!();
            iteration = 1;
        }

        if iteration == 1 {
            print!("{}", rect.0);
        } else {
            print!(", {}", rect.0);
        }

        iteration += 1;
    }

    println!();
}

fn str_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed reading input.");

    let input = input.trim();

    input.to_string()
}

fn int_input() -> i32 {
    let mut input = String::new();

    loop {

        io::stdin().read_line(&mut input).expect("Failed to read input.");

        let input = input.trim();

        match input.parse::<i32>() {
            Ok(input) => return input,
            Err(..) => println!("{} is not a valid number.", input),
        }
    }

}
