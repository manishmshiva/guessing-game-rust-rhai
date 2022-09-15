use rhai::{Engine, Scope, EvalAltResult};
use rand::Rng;
use std::io;

pub fn main() -> Result<(), Box<EvalAltResult>>{
    
    let engine = Engine::new();
    let mut scope = Scope::new();
    let target = rand::thread_rng().gen_range(1..=100);

    let ast = engine.compile_file("guess.rhai".into())?;

    println!("{}","Guess the number!");

    
    scope.push("target", target);

    loop{
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let guess: i32 = input
            .trim()
            .parse()
            .expect("Wanted a number");


        let result = engine.call_fn::<bool>(&mut scope, &ast, "guess", ( guess,) )?;

        if result {
            break
        };
    }
    
    Ok(())
}
