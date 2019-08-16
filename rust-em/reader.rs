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
            tokens: tokens,
        }
        .peekable()
    }
}

impl<'a> Iterator for Reader<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let pos = self.position;
        self.position = self.position + 1;

        match pos < self.tokens.len() {
            true => Some(self.tokens[pos]),
            false => None,
        }
    }
}

fn capture_token<'a>(s: &'a str, start: usize, end: usize, tokens: &mut Vec<&'a str> ) -> usize {
    if start < end {
        tokens.push(&s[start..end]);
    }

    return end;
}

fn tokenize(input: &str) -> Result<Vec<&str>, String> {
    let mut chars = input.char_indices().peekable();
    let mut pos = 0;
    let mut tokens = vec![];

    'outer: while let Some((x, c)) = chars.next()
    {
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
                        tokens.push(&input[pos..x+1]);
                        pos = x+1;
                        let _ = chars.by_ref().skip(1);
                    },
                    _ => {
                        tokens.push(&input[pos..pos+1]);
                        pos = pos + 1;
                    },
                }
            },
            '"' => {
                pos = capture_token(input, pos, x, &mut tokens);

                while let Some((x, nc)) = chars.by_ref().next() {
                    match nc {
                        '\\' => {
                            let _ = chars.next();
                        },
                        '"' => {
                            tokens.push(&input[pos..x+1]);
                            pos = x + 1;
                            continue 'outer;
                        },
                        _ => continue
                    }
                }
                return Err(format!("string not terminated: {}", &input[pos..])); 
            }
            _ if "[]{}()'`~^@".contains(c) => {
                pos = capture_token(input, pos, x, &mut tokens);

                tokens.push(&input[pos..pos+1]);
                pos = pos + 1;
            },
            _ if c.is_whitespace() || c == ',' => 
            {
                pos = capture_token(input, pos, x, &mut tokens);
                let _ = chars.by_ref().skip_while(|(_, c)| {
                    c.is_whitespace() || c == &','
                });

                if let Some((x, _)) = chars.by_ref().peek() {
                    pos = *x;
                }
            },
            _ => continue,
        }
    }

    Ok(tokens)
}

fn read_form<'a>(reader: &mut Peekable<Reader<'a>>) -> Result<MalType<'a>, String> {
    if let Some(token) = reader.next() {
        match token {
            "(" => return read_list(reader),
            _ => return read_atom(token),
        }
    }
    Err(String::from("reader is empty")) 
}

fn read_list<'a>(reader: &mut Peekable<Reader<'a>>) -> Result<MalType<'a>, String> {
    let mut list = vec!();

    while let Some(token) = reader.peek() {
        match token {
            &")" => {
                let _ = reader.next();
                return Ok(MalType::List(list))
            },
            _ => {
                let x = read_form(reader);
                list.push(x?);
            }
        }
    }
    return Err(String::from("matching closing brace not found"));
}

fn read_atom(token: &str) -> Result<MalType<'_>, String> {
    if let Ok(b) = token.parse::<bool>() {
        Ok(MalType::Bool(b))
    } else if let Ok(i) = token.parse::<i64>() {
        Ok(MalType::Int(i))
    } else if token == "nil" {
        Ok(MalType::Nil())
    } else {
        Ok(MalType::Symbol(token))
    }
}
