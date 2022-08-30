use std::env;

struct Rule {
    number: u8,
    message: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut start: u8 = 1;
    if args.len() >= 2 {
        if let Ok(x) = args[1].parse() {
            start = x;
        }
    }

    let mut end: u8 = 101;
    if args.len() >= 3 {
        if let Ok(x) = args[2].parse() {
            end = x;
            end += 1;
        }
    }

    for n in start..end {
        print!("{}: ", n);

        let rules = vec![
            Rule {
                number: 3,
                message: "fizz".to_owned(),
            },
            Rule {
                number: 5,
                message: "buzz".to_owned(),
            },
        ];

        let mut rule_applied = false;

        for r in rules {
            if n % r.number == 0 {
                print!("{}", r.message);
                rule_applied = true;
            }
        }

        if !rule_applied {
            print!("{}", n);
        }

        print!("\n")
    }
}
