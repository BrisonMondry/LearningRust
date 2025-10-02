

fn main() {
    println!("----------------------------");
    println!("Example problems from Chapter 8 of the Rust book");
    let (med, mode) = med_mode::run(&[1, 1, 3, 3].to_vec());
    println!("Median: {}, Mode: {:?}", med, mode);
    println!("----------------------------");
}

mod med_mode {
    use std::collections::BTreeMap;

    pub fn run(numbers: &Vec<usize>) -> (f32, Vec<usize>) {
        if numbers.len() == 0 {
            println!("No mode or median for empty list");
            return (f32::NAN, vec![]);
        }
        
        let mut mode: Vec<usize> = [].to_vec();
        let mut median: f32 = f32::NAN;

        if numbers.len() == 1 {
            mode.push(numbers[0]);
            median = numbers[0] as f32;
            return (median, mode);
        }

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

        for (&value, &count) in &occurrences {
            if count == numbers.len() {
                mode.push(value);
                median = value as f32;
                return (median, mode);
            }

            med_count += count;
            if med_count == midpoint -1 && numbers.len() % 2 == 0 {
                avg = true;
            }
            if med_count >= midpoint && !found_median {
                if avg {
                    median = (value + trailing_med) as f32 / 2.0;
                    found_median = true;
                }
                else {
                    median = value as f32;
                    found_median = true;
                }
            }
            trailing_med = value;

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