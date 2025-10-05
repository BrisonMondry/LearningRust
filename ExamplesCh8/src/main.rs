fn main() {
    println!("----------------------------");
    println!("Example problems from Chapter 8 of the Rust book");
    let (med, mode) = med_mode::run(&[1, 1, 3].to_vec());
    println!("Median: {}, Mode: {:?}", med, mode);
    let pig_latin = igpay_atinlay::run("Hello, world!😃\nThis is an example.");
    println!("Pig Latin: {}", pig_latin);
    hr_people_dictionary::run();
    println!("Done");
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

    pub fn run() {
        // store employee names under their department in a HashMap
        let mut company: HashMap<String, Vec<String>> = HashMap::new();

        loop {
            let mut buffer = String::new();

            let stdin = io::stdin();
            stdin.read_line(&mut buffer).expect("Failed to read line");
            let parts: Vec<&str> = buffer.split_whitespace().collect();
            if parts.len() == 0 {
                println!("Invalid command. Type 'help' for instructions.");
                continue;
            }
            
            if parts[0].eq_ignore_ascii_case("help") {
                println!("To add an employee to a department, use the following format:");
                println!("Add <employee_name> to <department_name>");
                println!("To see all employees in a department, use the following format:");
                println!("print <department_name>");
                println!("To see all employees in the company, use the following format:");
                println!("print All");
                println!("To exit the program, type 'exit'");
            } else if parts[0].eq_ignore_ascii_case("exit") {
                break;
            } else if parts[0].eq_ignore_ascii_case("Add") {
                let i = parts.iter().position(|&r| r.eq_ignore_ascii_case("to"));
                if i.is_none() || i.unwrap() < 2 || i.unwrap() >= parts.len() - 1 {
                   println!("Invalid command. Type 'help' for instructions.");
                   continue;
                }
                let i = i.unwrap();
                let name = parts[1..i].join(" ");
                let department = parts[i + 1..].join(" ");
                let entry = company.entry(department).or_insert(vec![]);
                entry.push(name);
                entry.sort();
            }else if parts[0].eq_ignore_ascii_case("Print"){
                if parts.len() != 2 {
                    println!("Invalid command. Type 'help' for instructions.");
                    continue;
                }
                let department = parts[1].to_string();
                if department == "all" {
                    let mut message = String::new();
                    let mut departments: Vec<&String> = company.keys().collect();
                    departments.sort();
                    for dept in departments {
                        let mut employees = company.get(dept).unwrap().clone();
                        employees.sort();
                        message.push_str(&format!("{}:\n{}\n- - - - -\n", dept, employees.join("\n")));
                    println!("{}", message);
                }
                } else {
                    if let Some(employees) = company.get(&department) {
                        println!("{}:\n{}", department, employees.join("\n"));
                    } else {
                        println!("No such department: {}", department);
                    }
                }
            } 
            else {
                println!("Invalid command. Type 'help' for instructions.");
            }
        }
    }

}