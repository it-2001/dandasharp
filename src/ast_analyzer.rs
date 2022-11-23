pub mod analyzer {
    use std::collections::HashMap;

    use crate::ast_parser::ast_parser::*;
    use crate::lexer::tokenizer::Tokens;
    pub fn analyze(mut tokens: Vec<Tokens>, mut lines: Vec<(usize, usize)>, ast: Tree) {
    }
    fn analyze_struct(
        tokens: &Vec<Tokens>,
        idx: &mut usize,
        this: &Head,
    ) -> Result<HashMap<String, Branch>, AnalyzeErr> {
        let mut result = HashMap::new();
        for (i, param) in this.parameters.iter().enumerate() {
            
            result.insert(k, v);
        }
        Ok(result)
        //Err(AnalyzeErr::Placeholder)
    }
    fn analyze_scope(){

    }
    fn token_cmp(tree_element: Tokens, source_token: Tokens) -> bool {
        match tree_element {
            Tokens::String(ref txt) => match txt.as_str() {
                "'string" => {
                    if let Tokens::String(_) = source_token {
                        return true;
                    }
                    return false;
                }
                "'number" => {
                    if let Tokens::Number(_, _, _) = source_token {
                        return true;
                    }
                    return false;
                }
                "'text" => {
                    if let Tokens::Text(_) = source_token {
                        return true;
                    }
                    return false;
                }
                "'char" => {
                    if let Tokens::Char(_) = source_token {
                        return true;
                    }
                    return false;
                }
                _ => tree_element == source_token,
            },
            _ => tree_element == source_token,
        }
    }
    struct Branch {
        name: String,
        nodes: Vec<ParamType>,
    }
    enum ParamType {
        Array(Vec<BranchParam>),
        Primitive(BranchParam)
    }
    enum BranchParam {
        Primitive(Tokens),
        Object(Branch),
    }
    enum AnalyzeErr {
        Placeholder,
        /// expected 0, found 1
        Expected(Tokens, Tokens)
    }
}
