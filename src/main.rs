// lengt with the reference
// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// mutability
// fn main() {
//     let mut s1 = String::from("hello");
//      change(&mut s1);
//      println!("{}", s1);

// }
// fn change(s: &mut String) {
//     s.push_str(", world");
// }

// borrowing mutable
// fn main(){
//     let mut s1 = String::from("hello");
//     let r1 = &mut s1;
//     println!("{}",r1);
// }
//function to The first_word function has a &String as a parameter. We don’t want ownership, so this is fine. But what should we return? We don’t really have a way to talk about part of a string. However, we could return the index of the end of the word, indicated by a space. Let’s try that, as shown in Listing 4-7.
fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if(item == b' '){
            return &s[..i];
        }
    }
        &s[..];
    
}
fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    prinln!(the word is {}, s);
}
