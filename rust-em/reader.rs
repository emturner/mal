use std::iter::Peekable;

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

fn capture_token<'a>(s: &'a str, start: usize, end: usize, mut tokens: &mut Vec<&'a str> ) -> usize {
    if start < end {
        tokens.push(&s[start..end]);
    }

    return end;
}

fn tokenize(input: &str) -> Result<Vec<&str>, String> {
    let mut chars = input.char_indices().peekable();
    let mut pos = 0;
    let mut tokens = vec![];

    while let Some((x, c)) = chars.next()
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
                        let _ = chars.by_ref().skip(1);
                    },
                    _ => tokens.push(&input[pos..pos+1]),
                }
            },
            '"' => {
                pos = capture_token(input, pos, x, &mut tokens);

                while let Some((x, nc)) = chars.by_ref().next() {
                    match nc {
                        '\\' => {
                            let _ = chars.by_ref().skip(1);
                        },
                        '"' => {
                            tokens.push(&input[pos..x+1]);
                            break;
                        },
                        _ => continue
                    }
                }
                return Err(format!("string not terminated: {}", &input[pos..])); 
            }
            _ if "[]{}()'`~^@".contains(c) => {
                pos = capture_token(input, pos, x, &mut tokens);

                tokens.push(&input[pos..pos+1])
            },
            _ if c.is_whitespace() || c == ',' => pos = capture_token(input, pos, x, &mut tokens),
            _ => continue,
        }
    }

    Ok(tokens)
}
