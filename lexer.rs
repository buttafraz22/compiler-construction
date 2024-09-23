use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
enum TokenType {
    Keyword,
    Identifier,
    IntegerLiteral,
    FloatLiteral,
    Operator,
    Punctuator,
    Unknown,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: String,
}

impl Token {
    fn new(token_type: TokenType, value: String) -> Self {
        Token { token_type, value }
    }
}

struct LexicalAnalyzer {
    input: String,
    position: usize,
    keywords: HashMap<String, TokenType>,
}

impl LexicalAnalyzer {
    fn new(source: String) -> Self {
        let mut analyzer = LexicalAnalyzer {
            input: source,
            position: 0,
            keywords: HashMap::new(),
        };
        analyzer.init_keywords();
        analyzer
    }

    fn init_keywords(&mut self) {
        self.keywords
            .insert("hindsa".to_string(), TokenType::Keyword); // int
        self.keywords
            .insert("asharia".to_string(), TokenType::Keyword); // float
        self.keywords
            .insert("agar".to_string(), TokenType::Keyword); // if
        self.keywords
            .insert("phir".to_string(), TokenType::Keyword); // else
        self.keywords
            .insert("lekinagar".to_string(), TokenType::Keyword); // else if
        self.keywords
            .insert("jabtk".to_string(), TokenType::Keyword); // while
        self.keywords
            .insert("niklo".to_string(), TokenType::Keyword); // break
        self.keywords
            .insert("wapsi".to_string(), TokenType::Keyword); // return
        self.keywords
            .insert("irshaad".to_string(), TokenType::Keyword); // print
        self.keywords
            .insert("chalooo".to_string(), TokenType::Keyword); // continue
    }

    fn is_whitespace(c: char) -> bool {
        c == ' ' || c == '\t' || c == '\n' || c == '\r'
    }

    fn is_alpha(c: char) -> bool {
        c.is_ascii_alphabetic()
    }

    fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_alphanum(c: char) -> bool {
        LexicalAnalyzer::is_alpha(c) || LexicalAnalyzer::is_digit(c)
    }

    fn get_next_word(&mut self) -> String {
        let start = self.position;
        while self.position < self.input.len()
            && LexicalAnalyzer::is_alphanum(self.input.chars().nth(self.position).unwrap())
        {
            self.position += 1;
        }
        self.input[start..self.position].to_string()
    }

    fn get_next_number(&mut self) -> String {
        let start = self.position;
        let mut has_decimal = false;

        while self.position < self.input.len() {
            let current_char = self.input.chars().nth(self.position).unwrap();

            if current_char == '.' {
                if has_decimal {
                    break;
                }
                has_decimal = true;
            } else if !LexicalAnalyzer::is_digit(current_char) {
                break;
            }
            self.position += 1;
        }

        self.input[start..self.position].to_string()
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.position < self.input.len() {
            let current_char = self.input.chars().nth(self.position).unwrap();

            // Skip whitespace
            if LexicalAnalyzer::is_whitespace(current_char) {
                self.position += 1;
                continue;
            }

            // Identify keywords or identifiers
            if LexicalAnalyzer::is_alpha(current_char) {
                let word = self.get_next_word();
                let token_type = self
                    .keywords
                    .get(&word)
                    .cloned()
                    .unwrap_or(TokenType::Identifier);
                tokens.push(Token::new(token_type, word));
            }
            // Identify integer or float literals
            else if LexicalAnalyzer::is_digit(current_char) {
                let number = self.get_next_number();
                let token_type = if number.contains('.') {
                    TokenType::FloatLiteral
                } else {
                    TokenType::IntegerLiteral
                };
                tokens.push(Token::new(token_type, number));
            }
            // Identify operators
            else if "+-*/=!".contains(current_char) {
                tokens.push(Token::new(TokenType::Operator, current_char.to_string()));
                self.position += 1;
            }
            // Identify punctuators
            else if "(){};".contains(current_char) {
                tokens.push(Token::new(TokenType::Punctuator, current_char.to_string()));
                self.position += 1;
            }
            // Handle unknown characters
            else {
                tokens.push(Token::new(TokenType::Unknown, current_char.to_string()));
                self.position += 1;
            }
        }

        tokens
    }
}

fn get_token_type_name(token_type: &TokenType) -> &str {
    match token_type {
        TokenType::Keyword => "KEYWORD",
        TokenType::Identifier => "IDENTIFIER",
        TokenType::IntegerLiteral => "INTEGER_LITERAL",
        TokenType::FloatLiteral => "FLOAT_LITERAL",
        TokenType::Operator => "OPERATOR",
        TokenType::Punctuator => "PUNCTUATOR",
        TokenType::Unknown => "UNKNOWN",
    }
}

fn print_tokens(tokens: &[Token]) {
    for token in tokens {
        println!(
            "Type: {}, Value: {}",
            get_token_type_name(&token.token_type),
            token.value
        );
    }
}

fn main() {
    let source_code = "hindsa x = 10; asharia y = 20.5; agar (x > y) { nikl ao; }".to_string();
    let mut lexer = LexicalAnalyzer::new(source_code.clone());
    let tokens = lexer.tokenize();
    println!("\n\n\nSource Code: {} \n\n\n", source_code);
    print_tokens(&tokens);
}
