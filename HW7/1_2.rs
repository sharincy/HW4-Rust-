use std::env;

pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
        let len = arr.len();
        let mut swapped;

        for _ in 0..len {
            swapped = false;

            for j in 0..len - 1 {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                    swapped = true;
                }
            }

            if !swapped {
                break;
            }
        }
    }

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <asc|desc> <num1> <num2> ...", args[0]);
        std::process::exit(1);
    }

    let order = &args[1];
    let is_ascending = match order.as_str() {
        "asc" => true,
        "desc" => false,
        _ => {
            eprintln!("Invalid sorting order. Use 'asc' or 'desc'.");
            std::process::exit(1);
        }
    };

    let mut numbers: Vec<i32> = args[2..]
        .iter()
        .map(|arg| {
            arg.parse().unwrap_or_else(|_| {
                eprintln!("Invalid number: {}", arg);
                std::process::exit(1);
            })
        })
        .collect();

    if is_ascending {
        bubble_sort(&mut numbers);
    } else {
        bubble_sort(&mut numbers);
        numbers.reverse();
    }

    for num in &numbers {
        println!("{}", num);
    }
}




#[test]
fn test_bubble_sorting_asc() {
    let mut input = vec![5, 2, 9, 1, 5];
    let expected = vec![1, 2, 5, 5, 9];
    bubble_sort(&mut input);
    assert_eq!(input, expected);
}

#[test]
fn test_bubble_sorting_desc() {
    let mut input = vec![5, 2, 9, 1, 5];
    let expected = vec![9, 5, 5, 2, 1];
    bubble_sort(&mut input);
    input.reverse();
    assert_eq!(input, expected);
}
