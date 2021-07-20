
use std::collections::HashMap;

mod dom;

fn main() {
    let mut attrs: HashMap<String, String> = HashMap::new();
    attrs.insert("bgcolor".to_string(),"skyblue".to_string());
    let tree1 = dom::elem("html".to_string(), HashMap::new(),
        vec![
            dom::elem("head".to_string(), HashMap::new(),
                vec![dom::elem("title".to_string(), HashMap::new(),
                    vec![dom::text("Title of the page".to_string())])]),
            dom::elem("body".to_string(), attrs,
                vec![dom::elem("p".to_string(), HashMap::new(),
                    vec![dom::text("A paragraph here lol".to_string())])])
        ]);
    
    println!("{}", tree1);
}
