fn main() {
    println!("----------------------------");
    println!("Example problems from Chapter 8 of the Rust book");
    let (med, mode) = med_mode::run(&[1, 1, 3].to_vec());
    println!("Median: {}, Mode: {:?}", med, mode);
    let pig_latin = igpay_atinlay::run("Hello, world!😃\nThis is an example.");
    println!("Pig Latin: {}", pig_latin);
    println!("----------------------------");
}

// Module to calculate median and mode of a list of numbers
// Separated into modules to organize in terms of book examples
// This is more complicated than it needs to be for the sake of
// pushing myself to use more Rust, and not just to use std lib functions
mod med_mode {
    use std::collections::BTreeMap;

    pub fn run(numbers: &Vec<usize>) -> (f32, Vec<usize>) {
        //edge case 1: empty list
        if numbers.len() == 0 {
            println!("No mode or median for empty list");
            return (f32::NAN, vec![]);
        }

        //initilize return values
        let mut mode: Vec<usize> = [].to_vec();
        let mut median: f32 = f32::NAN;

        //edge case 2: list of one number
        if numbers.len() == 1 {
            mode.push(numbers[0]);
            median = numbers[0] as f32;
            return (median, mode);
        }

        //count occurrences
        let mut occurrences: BTreeMap<usize, usize> = BTreeMap::new();
        for value in numbers {
            let count = occurrences.entry(*value).or_insert(0);
            *count += 1;
        }

        let mut mode_count = 0;

        let midpoint = numbers.len() / 2;
        let mut med_count = 0;
        let mut trailing_med = 0;
        let mut found_median = false;
        let mut avg = false;

        //calculate median
        for (&value, &count) in &occurrences {
            if count == numbers.len() {
                mode.push(value);
                median = value as f32;
                return (median, mode);
            }
            med_count += count;

            if med_count - 1 >= midpoint && !found_median {
                if avg {
                    median = (value + trailing_med) as f32 / 2.0;
                    found_median = true;
                } else {
                    median = value as f32;
                    found_median = true;
                }
            }
            trailing_med = value;

            if med_count == midpoint && numbers.len() % 2 == 0 {
                avg = true;
            }

            //calculate mode
            if count > mode_count {
                mode.clear();
                mode.push(value);
                mode_count = count;
            } else if count == mode_count {
                mode.push(value);
            }
        }

        (median, mode)
    }
}

mod igpay_atinlay {
    pub fn run(input: &str) -> String {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        let mut output = String::new();

        enum State {
            Whitespace,
            Word,
            VWord,
        }

        let mut current_state = State::Whitespace;
        let mut ch: char = ' ';

        for c in input.chars() {
            match current_state {
                State::Whitespace => {
                    if c.is_alphabetic() {
                        if vowels.contains(&ch) {
                            output.push(c);
                            current_state = State::VWord;
                        } else {
                            ch = c;
                            current_state = State::Word;
                        }
                    } else {
                        output.push(c);
                    }
                }
                State::Word => {
                    if c.is_alphabetic() {
                        output.push(c);
                    } else {
                        output.push(ch);
                        output.push_str("ay");
                        output.push(c);
                        current_state = State::Whitespace;
                    }
                }
                State::VWord => {
                    if c.is_alphabetic() {
                        output.push(c);
                    } else {
                        output.push_str("hay");
                        output.push(c);
                        current_state = State::Whitespace;
                    }
                }
            }
           
        }
        return output;
    }
}

mod hr_people_dictionary {
    use std::collections::HashMap;
    use std::io;

    // store employee names under their department in a HashMap
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    pub fn run() {
        loop {
            let line = io::stdin.read_line().expect("Failed to read line");
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            let mut name = String::new();
            let mut department = String::new();
            if parts[0] == String::from("help") {
                println!("To add an employee to a department, use the following format:");
                println!("Add <employee_name> to <department_name>");
                println!("To see all employees in a department, use the following format:");
                println!("<department_name>");
                println!("To see all employees in the company, use the following format:");
                println!("All");
                println!("To exit the program, type 'exit'");
            } else if parts[0] == String::from("exit") {
                break;
            } else if parts[0] == String::from("Add") {
               let i = parts.iter().position(|&r| r == "to");
               if i.is_none() || i.unwrap() < 2 || i.unwrap() >= parts.len() - 1 {
                   println!("Invalid command. Type 'help' for instructions.");
                   continue;
               }
            } else {
                println!("Invalid command. Type 'help' for instructions.");
            }
        }
    }

}