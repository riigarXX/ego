use super::ScopesStack;
use crate::ast::{AstNode, AstTokenType, AstTree};

pub struct Interpreter {
    scopes: ScopesStack,
    ast: AstTree,
}

impl Interpreter {
    pub fn new(scopes: ScopesStack, ast: AstTree) -> Interpreter {
        Interpreter { scopes, ast }
    }

    pub fn execute(&mut self) {
        self.execute_node(self.ast.root.clone());
    }

    fn execute_node(&mut self, node: AstNode) {
        match node.token_type {
            AstTokenType::Root => {
                for child in node.children {
                    self.execute_node(child);
                }
            }
            AstTokenType::FunctionCall => {
                if node.value == "print" {
                    self.execute_print(node);
                }
            }
            AstTokenType::VariableDeclaration => {
                if let Some((identifier, value)) = self.variable_declaration(node) {
                    let new_scopes =
                        ScopesStack::add_identifier(self.scopes.get(), identifier, value);

                    // update scopes with new variable
                    if let Some(new_scopes) = new_scopes {
                        self.scopes = new_scopes;
                    }
                }
            }
            _ => {}
        }
    }

    fn execute_print(&self, node: AstNode) {
        let string_node = &node.children[0];
        let mut string_chars = string_node.value.chars();
        string_chars.next();
        string_chars.next_back();
        let string_literal = string_chars.as_str();

        match &string_node.token_type {
            AstTokenType::StringLiteral => {
                println!("{}", string_literal);
            }
            _ => {}
        }
    }

    fn variable_declaration(&self, node: AstNode) -> Option<(String, String)> {
        let mut current = 0;
        let mut identifier = None;
        let mut value = None;

        while current < node.children.len() {
            match node.children[current].token_type {
                AstTokenType::Identifier => identifier = Some(node.children[current].value.clone()),
                AstTokenType::StringLiteral => value = Some(node.children[current].value.clone()),
                _ => {}
            }
            current += 1;
        }

        if identifier.is_some() && value.is_some() {
            Some((identifier.unwrap(), value.unwrap()))
        } else {
            None
        }
    }
}
