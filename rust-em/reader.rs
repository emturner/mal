use std::iter::Peekable;
use types::MalType;

pub fn read_str(input: &str) -> Result<MalType, String> {
    read_form(&mut Reader::new(tokenize(input)?))
}

struct Reader<'a> {
    position: usize,
    tokens: Vec<&'a str>,
}

impl Reader<'_> {
    fn new(tokens: Vec<&str>) -> Peekable<Reader> {
        Reader {
            position: 0,
            tokens,
        }
        .peekable()
    }
}

impl<'a> Iterator for Reader<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let pos = self.position;
        self.position += 1;

        if pos < self.tokens.len() {
            Some(self.tokens[pos])
        } else {
            None
        }
    }
}

fn capture_token<'a>(s: &'a str, start: usize, end: usize, tokens: &mut Vec<&'a str>) -> usize {
    if start < end {
        tokens.push(&s[start..end]);
    }

    end
}

fn tokenize(input: &str) -> Result<Vec<&str>, String> {
    let mut chars = input.char_indices().peekable();
    let mut pos = 0;
    let mut tokens = vec![];

    'outer: while let Some((x, c)) = chars.next() {
        match c {
            ';' => {
                pos = capture_token(input, pos, x, &mut tokens);

                tokens.push(&input[pos..]);
                return Ok(tokens);
            }
            '~' => {
                pos = capture_token(input, pos, x, &mut tokens);

                match chars.peek() {
                    Some((x, '@')) => {
                        tokens.push(&input[pos..=*x]);
                        pos = x + 1;
                        let _ = chars.by_ref().skip(1);
                    }
                    _ => {
                        tokens.push(&input[pos..=pos]);
                        pos += 1;
                    }
                }
            }
            '"' => {
                pos = capture_token(input, pos, x, &mut tokens);

                while let Some((x, nc)) = chars.by_ref().next() {
                    match nc {
                        '\\' => {
                            let _ = chars.next();
                        }
                        '"' => {
                            tokens.push(&input[pos..=x]);
                            pos = x + 1;
                            continue 'outer;
                        }
                        _ => continue,
                    }
                }
                return Err(String::from("(EOF|end of input|unbalanced)"));
            }
            _ if "[]{}()'`~^@".contains(c) => {
                pos = capture_token(input, pos, x, &mut tokens);

                tokens.push(&input[pos..=pos]);
                pos += 1;
            }
            _ if c.is_whitespace() || c == ',' => {
                pos = capture_token(input, pos, x, &mut tokens);
                let _ = chars
                    .by_ref()
                    .skip_while(|(_, c)| c.is_whitespace() || c == &',');

                if let Some((x, _)) = chars.by_ref().peek() {
                    pos = *x;
                }
            }
            _ => continue,
        }
    }

    Ok(tokens)
}

fn read_form<'a>(reader: &mut Peekable<Reader<'a>>) -> Result<MalType, String> {
    if let Some(token) = reader.next() {
        match token {
            "(" => return read_list(reader),
            "[" => return read_vector(reader),
            _ => return read_atom(token),
        }
    }
    Err(String::from("BLAH"))
}

fn read_list<'a>(reader: &mut Peekable<Reader<'a>>) -> Result<MalType, String> {
    let mut list = vec![];

    while let Some(token) = reader.peek() {
        match *token {
            ")" => {
                let _ = reader.next();
                return Ok(MalType::List(list));
            }
            _ => {
                let x = read_form(reader);
                list.push(x?);
            }
        }
    }
    Err(String::from("(EOF|end of input|unbalanced"))
}

fn read_vector<'a>(reader: &mut Peekable<Reader<'a>>) -> Result<MalType, String> {
    let mut vector = vec![];

    while let Some(token) = reader.peek() {
        match *token {
            "]" => {
                let _ = reader.next();
                return Ok(MalType::Vector(vector));
            }
            _ => {
                let x = read_form(reader);
                vector.push(x?);
            }
        }
    }
    Err(String::from("(EOF|end of input|unbalanced"))
}

fn read_atom(token: &str) -> Result<MalType, String> {
    if let Ok(b) = token.parse::<bool>() {
        Ok(MalType::Bool(b))
    } else if let Ok(i) = token.parse::<i64>() {
        Ok(MalType::Int(i))
    } else if token == "nil" {
        Ok(MalType::Nil())
    } else {
        Ok(MalType::Symbol(token.to_string()))
    }
}
