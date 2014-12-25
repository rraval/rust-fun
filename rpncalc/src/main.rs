#[deriving(Show, PartialEq, Eq)]
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

    // cannot use FromStr trait because it does not let the produced value have
    // a lifetime dependency on the slice passed in
    pub fn from_str<'a>(s: &'a str) -> Option<Token<'a>> {
        Token::from_str_with_unread(s).map(|(t, _)| t)
    }
}

#[test]
fn test_token_from_str() {
    assert_eq!(
        Token::from_str("1234").unwrap(),
        Token::Number(1234)
    );

    assert_eq!(
        Token::from_str("1234  +324").unwrap(),
        Token::Number(1234)
    );

    assert_eq!(
        Token::from_str("abc1213 12 + 5").unwrap(),
        Token::Unknown("abc1213")
    );
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

#[test]
fn test_token_iterator() {
    assert_eq!(
        TokenIterator("  1234 * 5678 + - abc /  ").collect::<Vec<Token>>(),
        vec!(
            Token::Number(1234),
            Token::Asterisk,
            Token::Number(5678),
            Token::Plus,
            Token::Minus,
            Token::Unknown("abc"),
            Token::Slash,
        )
    );
}

fn eval<'a, I: Iterator<Token<'a>>>(tokens: &mut I) -> Option<int> {
    let mut stack = Vec::<int>::new();

    for token in *tokens {
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

#[test]
fn test_eval() {
    assert_eq!(
        eval(
            &mut vec!(
                Token::Number(1234),
                Token::Number(4321),
                Token::Plus,
            ).into_iter()
        ).unwrap(),
        5555
    );
}

fn main() {
}
