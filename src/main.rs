
// use std::collections::HashMap;

use std::fs;

mod dom;
mod html_parser;
mod css;
mod css_parser;

fn main() {
    // let mut attrs: HashMap<String, String> = HashMap::new();
    // attrs.insert("bgcolor".to_string(),"skyblue".to_string());
    // let tree1 = dom::elem("html".to_string(), HashMap::new(),
    //     vec![
    //         dom::elem("head".to_string(), HashMap::new(),
    //             vec![dom::elem("title".to_string(), HashMap::new(),
    //                 vec![dom::text("Title of the page".to_string())])]),
    //         dom::elem("body".to_string(), attrs,
    //             vec![
    //                     dom::elem("p".to_string(), HashMap::new(),
    //                         vec![dom::text("A paragraph here lol".to_string())]),
    //                     dom::comment("This is a comment. Above this, we've got a p element hmm".to_string())
    //                 ])
    //     ]);
    
    // println!("{}", tree1);

    // println!("{}",parser::parse(fs::read_to_string("test.html").unwrap()));

    println!("{}",css_parser::parse(fs::read_to_string("test.css").unwrap()));
}
