#[derive(Debug)]
enum Error {
    UnexpectedEndOfInput,
    NotANumberChar(char),
    NotASymbolChar(char),
    NumberParseError(std::num::ParseIntError),
}

struct MinusExpression {
    left: i64,
    right: i64,
}

enum Symbol {
    Plus,
    Minus,
}

enum ParseTree {
    PlusExpr(i64, i64),
    MinusExpr(MinusExpression),
}

fn read_any_amount_of_whitespace(input: &str) -> &str {
    match input.chars().nth(0) {
        Some(' ') => read_any_amount_of_whitespace(&input[1..]),
        _ => input,
    }
}

fn read_number(input: &str) -> Result<(i64, &str), Error> {
    let mut chars = input.chars();
    let mut bytes_read = 0;
    loop {
        match chars.next() {
            // Allow a first char of -
            Some('-') if bytes_read == 0 => {
                bytes_read += 1;
            }
            Some(c) if c.is_digit(10) => {
                bytes_read += 1;
            }
            Some(' ') => break,
            Some(c) => return Err(Error::NotANumberChar(c)),
            None if bytes_read == 0 => return Err(Error::UnexpectedEndOfInput),
            None => break,
        }
    }
    let number = input[0..bytes_read]
        .parse()
        .map_err(Error::NumberParseError)?;
    Ok((number, &input[bytes_read..]))
}

fn read_symbol(input: &str) -> Result<(Symbol, &str), Error> {
    let symbol = match input.chars().nth(0) {
        Some('+') => Symbol::Plus,
        Some('-') => Symbol::Minus,
        Some(c) => return Err(Error::NotASymbolChar(c)),
        None => return Err(Error::UnexpectedEndOfInput),
    };
    Ok((symbol, &input[1..]))
}

fn parse_input(input: &str) -> Result<ParseTree, Error> {
    let left_trimmed = read_any_amount_of_whitespace(input);
    let (left_number, symbol_untrimmed) = match read_number(left_trimmed) {
        Ok(values) => values,
        Err(e) => return Err(e),
    };

    let symbol_trimmed = read_any_amount_of_whitespace(symbol_untrimmed);
    let (symbol, right_untrimmed) = read_symbol(symbol_trimmed)?;

    let right_trimmed = read_any_amount_of_whitespace(right_untrimmed);
    let (right_number, _) = read_number(right_trimmed)?;

    let tree = match symbol {
        Symbol::Plus => ParseTree::PlusExpr(left_number, right_number),
        Symbol::Minus => {
            let minus_expr = MinusExpression {
                left: left_number,
                right: right_number,
            };
            ParseTree::MinusExpr(minus_expr)
        }
    };
    Ok(tree)
}

fn printed_result(parse_tree: ParseTree) -> String {
    match parse_tree {
        ParseTree::PlusExpr(left, right) => format!("{} + {} = {}", left, right, left + right),
        ParseTree::MinusExpr(info) => format!(
            "{} - {} = {}",
            info.left,
            info.right,
            info.left - info.right
        ),
    }
}

fn main() {
    let parsed = match std::env::args().nth(1) {
        Some(string) => parse_input(&string),
        None => Err(Error::UnexpectedEndOfInput),
    };
    match parsed {
        Ok(parsed) => println!("{}", printed_result(parsed)),
        Err(e) => eprintln!("Something went wrong: {:?}", e),
    }
}
