use std::{collections::HashMap, io::Read};

use egui::PopupCloseBehavior;
use typst_syntax::{
    SyntaxNode,
    ast::{Arg, AstNode, ContentBlock, FuncCall, Ident},
    parse,
};
const SLIDE_FUNC_NAME: &str = "ltu-slide";
fn main() {
    let mut f = std::fs::File::open("/Users/pawel/toys/schmides/slides/example.typ").unwrap();

    let mut buffer = String::new();

    f.read_to_string(&mut buffer).ok();

    let ast = parse(&buffer);
    let slides = get_slide_items(&ast);
    let body_nodes = slides.body_nodes();

    for (i, c) in ast.children().enumerate() {
        if let Some(f) = c.cast::<FuncCall>() {
            let callee = f.callee();
            let u = callee.to_untyped();
            if let Some(i) = u.cast::<Ident>() {
                if i.as_str() == "ltu-slide" {
                    println!("slide");
                    for a in f.args().items() {
                        if let Arg::Pos(e) = a {
                            let e = e.to_untyped();
                            if let Some(c) = e.cast::<ContentBlock>() {
                                for e in c.body().exprs() {
                                    let e = e.to_untyped();
                                    if let Some(f) = e.cast::<FuncCall>() {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

struct SlideNodes<'a> {
    nodes: Vec<FuncCall<'a>>,
}
struct BodyNodes<'a> {
    nodes: Vec<ContentBlock<'a>>,
}
fn get_slide_items(ast: &SyntaxNode) -> SlideNodes {
    let nodes = get_named_calls(ast, SLIDE_FUNC_NAME);
    SlideNodes { nodes }
}

impl<'a> SlideNodes<'a> {
    fn body_nodes(self) -> BodyNodes<'a> {
        let mut nodes = vec![];
        for n in self.nodes {
            for arg in n.args().items() {
                if let Arg::Pos(expr) = arg {
                    let expr = expr.to_untyped();
                    if let Some(cb) = expr.cast::<ContentBlock>() {
                        nodes.push(cb);
                    }
                }
            }
        }
        BodyNodes { nodes }
    }
}

struct Notes<'a>(HashMap<usize, &'a str>);

impl<'a> BodyNodes<'a> {
    fn notes(self) -> Notes<'a> {
        let notes = HashMap::new();

        Notes(notes)
    }
}

fn get_named_calls<'a>(node: &'a SyntaxNode, name: &str) -> Vec<FuncCall<'a>> {
    let mut calls: Vec<FuncCall> = vec![];
    for c in node.children() {
        if let Some(f) = c.cast::<FuncCall>() {
            if let Some(i) = f.callee().to_untyped().cast::<Ident>() {
                if i.as_str() == name {
                    calls.push(f);
                }
            }
        }
    }
    calls
}
