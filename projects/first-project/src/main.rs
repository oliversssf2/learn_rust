fn main() {
    println!("Hello, world!");
    
    let mut s = String::from("hello world");

    let word = first_word(&s);
    
    s.clear();
    
    println!("the first word is: {}", word);
    // let word = 
   
    // s[3] = 'j';
}

fn first_word(s: &String) -> &str {
let bytes = s.as_bytes();
for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}







