#[deriving(Show)]
enum Token<'u> {
    Number(int),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Unknown(&'u str),
}

impl<'u> Token<'u> {
    pub fn from_str_with_unread<'a>(s: &'a str) -> Option<(Token<'a>, &'a str)> {
        if let Some(c) = s.chars().nth(0) {
            match c {
                '+' => return Some((Token::Plus, s.slice_from(1))),
                '-' => return Some((Token::Minus, s.slice_from(1))),
                '*' => return Some((Token::Asterisk, s.slice_from(1))),
                '/' => return Some((Token::Slash, s.slice_from(1))),

                _ => {}
            }

            if !c.is_digit(10) {
                // consume up to a delimiter
                let offset = s.chars().position(|x|
                    x == '+' || x == '-' ||
                    x == '*' || x == '/' ||
                    x.is_whitespace()
                ).unwrap_or(s.len());

                return Some((
                    Token::Unknown(s.slice_to(offset)),
                    s.slice_from(offset),
                ));
            }

            let mut accumulator = 0i;
            let mut next_consume = 0u;

            for (i, c) in s.chars().enumerate() {
                if let Some(d) = c.to_digit(10) {
                    accumulator *= 10;
                    accumulator += d as int;
                    next_consume = i + 1;
                } else {
                    break;
                }
            }

            return Some((
                Token::Number(accumulator),
                s.slice_from(next_consume),
            ));
        }

        return None;
    }
}

struct TokenIterator<'s>(&'s str);

impl<'a> Iterator<Token<'a>> for TokenIterator<'a> {
    fn next(&mut self) -> Option<Token<'a>> {
        if let Some((token, slice)) = Token::from_str_with_unread(self.0.trim_left()) {
            self.0 = slice;
            return Some(token);
        }

        return None;
    }

    fn size_hint(&self) -> (uint, Option<uint>) {
        (0, Some(self.0.chars().count()))
    }
}

fn eval(calc: &str) -> Option<int> {
    let mut stack: Vec<int> = Vec::new();

    for token in TokenIterator(calc) {
        match token {
            Token::Number(n) => stack.push(n),

            Token::Plus => {
                let n1 = stack.pop().unwrap();
                let n2 = stack.pop().unwrap();
                stack.push(n1 + n2);
            },

            _ => {},
        }
    }

    return stack.pop();
}

fn main() {
    println!("{}", eval("   1234 6789 +"));
}
