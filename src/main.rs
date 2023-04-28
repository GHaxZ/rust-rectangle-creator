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

    fn change(&mut self, height: i32, width: i32) {
        self.height = height;
        self.width = width;
    }
}

fn main() {
    let mut rects = HashMap::<String, Rectangle>::new();

    loop {
        println!("\nEnter a command (new, show, draw, change, exit): ");

        let command = str_input().to_lowercase().replace(" ", "");

        if command == "new" {
            new_rect(&mut rects);
        } else if command == "show" {
            print_rects(&rects);
        } else if command == "draw" {
            draw_rect(&rects);
        } else if command == "change"{
            change_rect(&mut rects);
        } else if command == "exit" {
            break;
        } else {
            println!("\"{}\" is not valid command.", command);
        }
    }
}

fn new_rect(rects: &mut HashMap<String, Rectangle>){
    let dimensions = get_dimensions();

    let name = loop {
        println!("\nGive your rectangle a name: ");

        let name = str_input();

        if rects.contains_key(&name) {
            println!("\n{} is already used.", name);
        } else {
            break name
        }
    };


    let rectangle = Rectangle {
        height: *dimensions.get(0).unwrap(),
        width: *dimensions.get(1).unwrap(),
    };

    println!("\nYou successfully created a new rectangle called {}.", name);

    rects.insert(name, rectangle);
}

fn draw_rect(rects: &HashMap<String, Rectangle>) {
    if rects.len() < 1 {
        println!("\nThere are currently no rectangles to draw.");
        return;
    }

    loop {
        print_rects(&rects);

        println!("\nPlease enter the name of the rectangle you want to print: ");
        let input = str_input();

        if rects.contains_key(&input) {
            if let Some(rect) = rects.get(&input) {
                rect.draw();
                break;
            }
        }

        println!("\nNo rectangle found with the name {}", input);
    }
}

fn print_rects(rects: &HashMap<String, Rectangle>) {
    if rects.len() < 1 {
        println!("\nThere are currently no rectangles.");
        return;
    }

    let mut iteration = 1;
    println!("\nThese are your current rectangles: ");
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

fn change_rect(rects: &mut HashMap<String, Rectangle>){
    if rects.len() < 1 {
        println!("\nThere are currently no rectangles to modify.");
        return;
    }

    loop {
        print_rects(&rects);

        println!("\nPlease enter the name of the rectangle you want to modify: ");
        let input = str_input();

        if rects.contains_key(&input) {
            let dimensions = get_dimensions();
            rects.get_mut(&input).unwrap().change(dimensions[0], dimensions[0]);
            break;
        }

        println!("\nNo rectangle found with the name {}", input);
    };
}

fn get_dimensions() -> Vec<i32>{
    println!("\nEnter the height of the rectangle: ");
    let height = int_input();

    println!("\nEnter the width of the rectangle: ");
    let width = int_input();

    let dimensions: Vec<i32> = vec![height, width];
    dimensions
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
            Err(..) => println!("\n{} is not a valid number.", input),
        }
    }
}
