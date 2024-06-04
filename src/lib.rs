pub mod utils {
    use clap::Parser;

    #[derive(Parser, Debug)]
    #[command(version, about, long_about = None)]
    struct Args {
        #[arg(value_name("number"))]
        number: String,
    }

    #[derive(Debug)]
    pub enum Base {
        Binary,
        Octal,
        Decimal,
        Hexadecimal,
    }

    pub fn tokenize_number(input: &str) -> Option<(String, Base, i8)> {
        let mut is_negative = 1;
        let mut base = Base::Decimal;
        let mut chars = 0;

        let mut text = input.chars().peekable();

        if let Some(c) = text.peek() {
            match c {
                '-' => {
                    is_negative = -1;
                    text.next();
                    chars += 1;
                }
                '+' => {
                    text.next();
                    chars += 1;
                }
                _ => (),
            }
        } else {
            return None;
        };

        if let Some(c) = text.peek() {
            let number = match c {
                '0' => {
                    text.next(); // Have to next here to peek second digit
                    if let Some(c) = text.peek() {
                        chars += 1;
                        match c {
                            'b' => {
                                base = Base::Binary;
                                eat_binary_digits(&input[chars + 1..])
                            }
                            'o' => {
                                base = Base::Octal;
                                eat_octal_digits(&input[chars + 1..])
                            }
                            'x' => {
                                base = Base::Hexadecimal;
                                eat_hexadecimal_digits(&input[chars + 1..])
                            }
                            '0'..='9' => eat_decimal_digits(&input[chars + 1..]),
                            _ => None,
                        }
                    } else {
                        Some("0".to_string())
                    }
                }
                _ => eat_decimal_digits(&input[chars..]),
            };
            if let Some(number) = number {
                Some((number, base, is_negative))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn eat_decimal_digits(text: &str) -> Option<String> {
        let mut text = text.chars().peekable();
        let mut number = String::new();
        loop {
            if let Some(c) = text.clone().peek() {
                match c {
                    '_' | '`' => {
                        text.next();
                    }
                    c @ '0'..='9' => {
                        number.push(*c);
                        text.next();
                    }
                    _ => break,
                }
            } else {
                break;
            };
        }
        Some(number)
    }

    fn eat_binary_digits(text: &str) -> Option<String> {
        let mut text = text.chars().peekable();
        let mut number = String::new();
        loop {
            if let Some(c) = text.clone().peek() {
                match c {
                    '_' | '`' => {
                        text.next();
                    }
                    c @ '0'..='1' => {
                        text.next();
                        number.push(*c);
                    }
                    _ => break,
                }
            } else {
                break;
            };
        }
        Some(number)
    }

    fn eat_octal_digits(text: &str) -> Option<String> {
        let mut text = text.chars().peekable();
        let mut number = String::new();
        loop {
            if let Some(c) = text.clone().peek() {
                match c {
                    '_' | '`' => {
                        text.next();
                    }
                    c @ '0'..='7' => {
                        text.next();
                        number.push(*c);
                    }
                    _ => break,
                }
            } else {
                break;
            };
        }
        Some(number)
    }

    fn eat_hexadecimal_digits(text: &str) -> Option<String> {
        let mut text = text.chars().peekable();
        let mut number = String::new();
        loop {
            if let Some(c) = text.clone().peek() {
                match c {
                    '_' | '`' => {
                        text.next();
                    }
                    c @ ('0'..='9' | 'a'..='f' | 'A'..='F') => {
                        text.next();
                        number.push(*c);
                    }
                    _ => break,
                }
            } else {
                break;
            };
        }
        Some(number)
    }

    pub fn parse_number(number: &str) -> Option<i128> {
        if let Some((text, base, negative)) = tokenize_number(number) {
            let mut value: i128 = 0;

            let base = match base {
                Base::Decimal => 10,
                Base::Binary => 2,
                Base::Octal => 8,
                Base::Hexadecimal => 16,
            };

            for c in text.chars() {
                if let Some(val) = c.to_digit(base) {
                    value *= i128::from(base);
                    value += i128::from(val);
                } else {
                    return None;
                }
            }

            return Some(value * i128::from(negative));
        }
        return None;
    }

    pub fn number_in_base(number: i128, base: u128) -> String {
        let mut s = String::new();

        let (mut number, is_negative) = {
            if number < 0 {
                (-number as u128, true)
            } else {
                (number as u128, false)
            }
        };

        let digits = ('0'..='9').chain('A'..='F').collect::<Vec<char>>();

        if number == 0 {
            s.push('0');
        };

        while number != 0 {
            let digit = digits.get((number % base) as usize).unwrap();
            s.push(*digit);
            number /= base;
        }

        match base {
            2 => s.push_str("b0"),
            8 => s.push_str("o0"),
            16 => s.push_str("x0"),
            _ => (),
        };

        if is_negative {
            s.push('-');
        }

        return s.chars().rev().collect();
    }
}
