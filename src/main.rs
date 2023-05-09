mod tools;
use tools::lexer::BasicLexer;
use tools::lexer::Lexer;


fn main() {
    let mut line = "2 + 2 + token";
    println!("tokens to lex :");
    //let b1 = std::io::stdin().read_line(&mut line).unwrap();
    let lexer = BasicLexer::new(line.into());
    let tokens = lexer.lex();
    //println!("no of bytes read , {}", b1);

    for token in tokens {
        println!(

            "Token '{}' of type {} at position {}",
             token.text,
             token.tpe.to_string(),
             token.start_pos

        )
    }
}
