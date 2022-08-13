use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let number = match args.get(1) {
        Some(arg1) => { 
            if match arg1.as_str().get(1) {
               Some() =>,
               None => 
            } == "t" && arg1.len() == 3 {
                match args.get(2 as usize) {
                    Some(arg2) => Some(translate(&arg2, &arg2.get(0), &arg2.get(3))),
                    None => {
                        println!("Incorrect syntax");
                        None
                    },
                }
            }else{
                println!("Invalid argument {}", arg1);
                None
            }
        },
        None => {
            println!("Incorrect syntax");
            None
        },
    };
    match number {
        Some(number) => println!("Translated: {}", number),
        None => println!(),
    }
}
fn translate(keyword: &str, input_lang: &str, output_lang: &str) -> String{
   String::from(keyword) 
}