
use std::collections::HashMap;

use crate::dom::ElementData;
use crate::dom::Node;
use crate::dom::NodeType;
use crate::css::Stylesheet;
use crate::css::Selector;
use crate::css::SimpleSelector;
use crate::css::Specificity;
use crate::css::Rule;
use crate::css::Value;

struct StyledNode<'a> {
    node: &'a Node,
    specified_values: PropertyMap,
    children: Vec<StyledNode<'a>>,
}

type PropertyMap = HashMap<String, Value>;

fn matches(elem: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Selector::Simple(ref sel) => matches_simple_selector(elem, sel),
    }
}

fn matches_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {

    //check if tag doesnt match
    if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
        return false;
    }

    //check if id doesnt match
    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }

    //check if class doesnt match
    if selector.class.iter().any(|class| !elem.classes().contains(&**class)) {
        return false;
    }

    //no non-matching selectors found
    return true;
}

type MatchedRule<'a> = (Specificity, &'a Rule);

fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    rule.selectors.iter().find(|sel| matches(elem, sel)).map(|sel| (sel.specificity(), rule))
}

fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    stylesheet.rules.iter().filter_map(|rule| match_rule(elem, rule)).collect()
}

fn specified_values(elem: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut matched_rules = matching_rules(elem, stylesheet);
    matched_rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    for (_, rule) in matched_rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone() , declaration.value.clone());
        }
    }

    values
}

fn style_tree<'a>(root: &'a Node, stylesheet: &Stylesheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        specified_values: match &root.node_type {
            NodeType::Element(elem) => specified_values(&elem, stylesheet),
            _ => HashMap::new()
        },
        children: root.children.iter().map(|node| style_tree(node, stylesheet)).collect()
    }
}