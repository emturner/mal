use std::iter::Peekable;

struct Reader<'a> {
    position: usize,
    tokens: Vec<&'a str>
}

impl Reader<'_> {
    fn new(tokens: Vec<&str>) -> Peekable<Reader> {
        Reader {
            position: 0, 
            tokens: tokens
        }.peekable()
    }
}

impl<'a> Iterator for Reader<'a> {
   type Item = &'a str; 
   
   fn next(&mut self) -> Option<&'a str> {
       let pos = self.position;
       self.position = self.position + 1;
       
       match pos < self.tokens.len() {
           true => Some(self.tokens[pos]),
           false => None
       }
   }
}

fn tokenize(input: &str) -> Result<Vec<&str>, String> {

    Ok(vec!())
}
