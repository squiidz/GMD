use token::Token;

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct Branch {
    prefix: String,
    original: String,
    tokens: Vec<Token>,
    length: usize,
}

impl Branch {
    pub fn new(name: String) -> Result<Branch, String> {
        let length = name.len();
        let mut b = Branch{
            prefix: Branch::get_prefix(&name).unwrap(),
            original: name,
            tokens: Vec::new(),
            length: length
        };
        match b.get_tokens() {
            Ok(_) => return Ok(b),
            Err(e) => Err(e),
        }
    }

    pub fn parse(&self) -> String {
        let mut branch_name = String::new();
        let mut d_under: bool = false;

        for (i, t) in self.tokens.iter().enumerate() {
            let next: char = if (i+1) < self.tokens.len() {
                self.tokens[i+1].clone().into()
            } else {
                '\0'
            };
            match *t {
                Token::NUMBER(n) => {
                    branch_name.push(n);
                    if !next.is_numeric() && !d_under {
                        branch_name.push('_');
                        d_under = true;
                    }
                },
                Token::LETTER(l) => branch_name.push(l),
                _ => branch_name.push(t.clone().into()),
            }
        }
        branch_name
    }

    pub fn get_tokens(&mut self) -> Result<&[Token], String> {
        if self.tokens.len() == 0 {
            self.tokens =
                self.original.chars()
                .map(|c| Token::from(c))
                .collect::<Vec<Token>>();
        }
        Ok(&self.tokens)
    }

    fn get_prefix(name: &str) -> Option<String> {
        for (i, c) in name.chars().enumerate() {
            if c == ' ' {
                if name.chars().nth(i-1).unwrap().is_numeric() {
                    return Some(name.split_at(i).0.to_owned())
                }
            }
        }
        None
    }
}
