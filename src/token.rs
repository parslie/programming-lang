use std::str::Split;

#[derive(Debug, Clone)]
pub enum Token {
    Exit,
    IntLiteral(String),
    NewLine,
}

fn evaluate_buffer(buffer: &str) -> Token {
    println!("Evaluating '{}'..", buffer);

    if buffer == "exit" {
        return Token::Exit;
    }

    let mut is_numeric = true;
    for char in buffer.chars() {
        if !char.is_numeric() {
            is_numeric = false;
            break;
        }
    }

    if is_numeric {
        return Token::IntLiteral(buffer.to_string());
    } else {
        panic!("Buffer '{}' could not be evaluated!", buffer);
    }
}

fn tokenize_line(line: &str, tokens: &mut Vec<Token>) {
    println!("Tokenizing '{}'...", line);

    let mut chars = line.chars().peekable();
    let mut buffer = String::new();

    while let Some(ch) = chars.next() {
        let should_evaluate = if let Some(next_ch) = chars.peek() {
            next_ch.is_whitespace()
        } else {
            true
        };

        if !ch.is_whitespace() {
            buffer.push(ch);
        }

        if should_evaluate {
            let token = evaluate_buffer(&buffer);
            tokens.push(token);
            buffer.clear();
        }
    }

    tokens.push(Token::NewLine);
}

fn compose_line(line: &str, lines: &mut Split<'_, &str>) -> String {
    let mut line = String::from(line);

    while let Some('\\') = line.chars().last() {
        match lines.next() {
            Some(next_line) => line.push_str(next_line),
            None => break,
        }
    }

    return line.replace('\\', " ");
}

pub fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let split_by = if code.contains("\r\n") { "\r\n" } else { "\n" };
    let mut lines = code.split(split_by);

    while let Some(line) = lines.next() {
        let line = compose_line(&line, &mut lines);
        tokenize_line(&line, &mut tokens);
    }

    return tokens;
}
