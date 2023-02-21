// https://www.codewars.com/kata/52ffcfa4aff455b3c2000750

use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

use regex::{Match, Regex};

fn main() {
    let mut inputs = Vec::new();
    // inputs.push("x = 7"); // -> 7
    // inputs.push("x + 6"); // -> 13
    // inputs.push("y + 7"); // -> unknown variable y
    // inputs.push("x = y = 7"); // -> 7
    // inputs.push("x = 13 + (y = 3)"); // -> 16

    // inputs.push("fn avg => (x + y) / 2"); // -> ERROR: Unknown identifier 'x'
    // inputs.push("fn avg x y => (x + y) / 2");
    // inputs.push("a = 2"); // ->  2
    // inputs.push("b = 4"); // ->  4
    // inputs.push("avg a b"); // ->  3

    // inputs.push("fn echo x => x");
    // inputs.push("fn add x y => x + z"); // -> ERROR: Unknown identifier 'z'
    // inputs.push("fn add x y => x + y");
    // inputs.push("add echo 4 echo 3"); // -> 7

    // inputs.push("fn inc x => x + 1");
    // inputs.push("a = 0"); // -> 0
    // inputs.push("a = inc a"); // -> 1
    // inputs.push("fn inc x => x + 2");
    // inputs.push("a = inc a"); // -> 3

    // inputs.push("x = add 4 echo (y = 6)"); // -> 10
    // inputs.push("(3 + 4) * (5 - 6)"); // -> -7
    // inputs.push("sub 12 4"); // -> Cant resolve expression
    // inputs.push("   ");
    // inputs.push("avg 4 2 + avg 10 30");

    // inputs.push("x = 1"); // Ok(Some(1.0)));
    // inputs.push("fn avg x y => (x + y) / 2"); // Ok(None)
    // inputs.push("fn x => 0"); // name conflict
    // inputs.push("avg = 5"); // name conflict

    inputs.push("fn one => 1"); // Ok(Some(1.0)));
    inputs.push("one"); // Ok(Some(1.0)));

    // inputs.push("fn add x x => x + x");

    // inputs.push("fn f1 a1 a2 => a1 * a2");
    // inputs.push("fn f2 a1 a2 a3 => a1 * a2 * a3");
    // inputs.push("f2 f2 1 2 3 f1 4 5 f1 6 7");

    let mut interpreter = Interpreter::new();
    for input in inputs {
        println!("{:?} -> {:?}", input, interpreter.input(input));
    }
    println!("\n{:?}", interpreter.state);
}

#[derive(Debug, PartialEq)]
enum TokenType {
    Id(String),
    Num(f32),
    LParen,
    RParen,
    Function,
    FunctionArrow,
    Op(Operator),
    Whitespace,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Equals,
}

struct Token {
    regex: Regex,
    resolve_fn: fn(Match) -> TokenType,
}

impl Token {
    fn new(pattern: &str, resolve_fn: fn(Match) -> TokenType) -> Self {
        let mut full_pattern: String = "^".to_owned();
        full_pattern.push_str(pattern);
        Self {
            regex: Regex::new(full_pattern.as_str()).unwrap(),
            resolve_fn,
        }
    }

    fn resolve(&self, input: &str) -> Option<(TokenType, usize)> {
        match self.regex.find(&input) {
            Some(m) => Some(((self.resolve_fn)(m), m.range().len())),
            None => None,
        }
    }
}

struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    fn new() -> Self {
        Self {
            tokens: vec![
                Token::new(r"[a-zA-Z_]\w*", |m| TokenType::Id(String::from(m.as_str()))),
                Token::new(r"\d+(\.\d+)?", |m| TokenType::Num(m.as_str().parse::<f32>().unwrap())),
                Token::new(r"\(", |_| TokenType::LParen),
                Token::new(r"\)", |_| TokenType::RParen),
                Token::new(r"fn", |_| TokenType::Function),
                Token::new(r"=>", |_| TokenType::FunctionArrow),
                Token::new(r"\+", |_| TokenType::Op(Operator::Add)),
                Token::new(r"-", |_| TokenType::Op(Operator::Sub)),
                Token::new(r"\*", |_| TokenType::Op(Operator::Mul)),
                Token::new(r"/", |_| TokenType::Op(Operator::Div)),
                Token::new(r"%", |_| TokenType::Op(Operator::Mod)),
                Token::new(r"=", |_| TokenType::Op(Operator::Equals)),
                Token::new(r"\s+", |_| TokenType::Whitespace),
            ],
        }
    }

    fn scan(&self, input: &str) -> Result<Vec<TokenType>, &str> {
        let mut current = input;
        let mut tokens = Vec::new();
        while current.len() > 0 {
            let matches = self
                .tokens
                .iter()
                .map(|tk| (tk, tk.resolve(&current)))
                .filter(|(_, res)| res.is_some())
                .map(|(tr, res)| (tr, res.unwrap()));

            let longest_match = matches.max_by_key(|&(_, (_, len))| len);
            match longest_match {
                Some((_, (tk, len))) => {
                    tokens.push(tk);
                    current = &current[len..];
                }
                None => return Err("Invalid token"),
            };
        }
        // tokens.push(TokenType::EOL);
        Ok(tokens.into_iter().filter(|t| *t != TokenType::Whitespace).collect())
    }
}

#[derive(Debug, PartialEq)]
enum Statement {
    FnDef {
        name: String,
        params: Vec<String>,
        body: Expression,
    },
    Expression {
        expr: Expression,
    },
    Nop,
}

#[derive(Debug, PartialEq)]
enum Expression {
    Variable {
        id: String,
    },
    Number {
        value: f32,
    },
    Assignment {
        id: String,
        expr: Box<Expression>,
    },
    FnCall {
        fn_id: String,
        args: Vec<Box<Expression>>,
    },
    Operation {
        op: Operator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
}

struct Parser {
    precedence: HashMap<Operator, u8>,
    is_left_associative: HashMap<Operator, bool>,
    known_functions: HashMap<String, u8>,
}

impl Parser {
    fn new() -> Self {
        Parser {
            precedence: HashMap::from([
                (Operator::Mul, 2),
                (Operator::Div, 2),
                (Operator::Mod, 2),
                (Operator::Add, 1),
                (Operator::Sub, 1),
                (Operator::Equals, 0),
            ]),

            is_left_associative: HashMap::from([
                (Operator::Mul, true),
                (Operator::Div, true),
                (Operator::Mod, true),
                (Operator::Add, true),
                (Operator::Sub, true),
                (Operator::Equals, false),
            ]),

            known_functions: HashMap::new(),
        }
    }

    fn parse(&mut self, tokens: Vec<TokenType>) -> Result<Statement, String> {
        let mut token_deque = VecDeque::from(tokens);

        let statement = match token_deque.front() {
            Some(TokenType::Function) => {
                let fn_def = self.parse_function_def(&mut token_deque);
                if let Ok(Statement::FnDef {
                    name,
                    params: args,
                    body: _,
                }) = &fn_def
                {
                    self.known_functions.insert(name.into(), args.len() as u8);
                }
                fn_def
            }
            Some(_) => {
                let e = self.parse_expression(&mut token_deque)?;
                Ok(Statement::Expression { expr: e })
            }
            None => Ok(Statement::Nop),
        };

        if token_deque.is_empty() {
            statement
        } else {
            Err(String::from("Parser: Unaccounted tokens"))
        }
    }

    /// Parses a function definition seperately
    fn parse_function_def(&self, tokens: &mut VecDeque<TokenType>) -> Result<Statement, String> {
        // Keyword
        match tokens.pop_front() {
            Some(TokenType::Function) => {}
            _ => return Err(String::from("FnDef: Invalid function definition keyword")),
        }

        // Function name
        let fn_name: String;
        if let Some(TokenType::Id(id)) = tokens.pop_front() {
            fn_name = id;
        } else {
            return Err(String::from("FnDef: Function name expected"));
        }

        // Arguments
        let mut args: Vec<String> = Vec::new();
        loop {
            match tokens.pop_front() {
                Some(TokenType::Id(arg)) => args.push(arg),
                Some(TokenType::FunctionArrow) => break,
                _ => return Err(String::from("FnDef: Invalid token after args")),
            }
        }

        // Body
        let body = self.parse_expression(tokens)?;

        // Full fn def
        Ok(Statement::FnDef {
            name: fn_name,
            params: args,
            body,
        })
    }

    /// All other statements are expressions, use shunting yard
    fn parse_expression(&self, tokens: &mut VecDeque<TokenType>) -> Result<Expression, String> {
        let mut stack = Vec::new();
        let mut output = Vec::new();

        let mut index = 0;
        while tokens.len() > 0 {
            let token = tokens.pop_front().unwrap();
            index += 1;
            // println!("{:?}", token);
            match &token {
                TokenType::Num(x) => {
                    self.push_to_output(Expression::Number { value: *x }, index, &mut output, &mut stack)
                }
                TokenType::Id(v) => match self.known_functions.get(v) {
                    Some(_) => stack.push((token, index)),
                    None => self.push_to_output(Expression::Variable { id: v.into() }, index, &mut output, &mut stack),
                },
                TokenType::Op(op_token) => {
                    let token_prec = self.precedence.get(op_token).unwrap();
                    let token_is_left_assoc = *self.is_left_associative.get(op_token).unwrap();
                    while !stack.is_empty() {
                        match stack.pop().unwrap() {
                            (TokenType::Op(op_stack), _)
                                if token_is_left_assoc && token_prec <= self.precedence.get(&op_stack).unwrap() =>
                            {
                                let rhs = output.pop().unwrap().0;
                                let lhs = output.pop().unwrap().0;
                                self.push_to_output(
                                    Expression::Operation {
                                        op: op_stack,
                                        lhs: Box::new(lhs),
                                        rhs: Box::new(rhs),
                                    },
                                    index,
                                    &mut output,
                                    &mut stack,
                                )
                            }
                            t => {
                                stack.push(t);
                                break;
                            }
                        }
                    }
                    stack.push((token, index));
                }
                TokenType::LParen => stack.push((token, index)),
                TokenType::RParen => {
                    loop {
                        if stack.is_empty() {
                            return Err(String::from("Expr: Missing LParen"));
                        }
                        match self.handle_stack_token(&mut stack, &mut output) {
                            Ok(TokenType::LParen) => break,
                            Ok(_) => {}
                            Err(msg) => return Err(msg),
                        }
                    }

                    let (element, index) = output.pop().unwrap();
                    self.push_to_output(element, index, &mut output, &mut stack)
                }
                _ => return Err(String::from("Expr: Invalid token")),
            }
        }

        while !stack.is_empty() {
            match self.handle_stack_token(&mut stack, &mut output) {
                Ok(TokenType::LParen) => return Err(String::from("Expr: Missing RParen")),
                Ok(_) => {}
                Err(msg) => return Err(msg),
            }
        }

        match output.len() {
            1 => Ok(output.pop().unwrap().0),
            _ => Err(String::from("Expr: Couldn't reduce expression to single node")),
        }
    }

    fn handle_stack_token(
        &self,
        stack: &mut Vec<(TokenType, u32)>,
        output: &mut Vec<(Expression, u32)>,
    ) -> Result<TokenType, String> {
        let (token, stack_index) = stack.pop().unwrap();

        match &token {
            TokenType::LParen => {}
            TokenType::Num(x) => self.push_to_output(Expression::Number { value: *x }, stack_index, output, stack),
            TokenType::Op(op_stack) => {
                if output.len() < 2 {
                    return Err(String::from("Expr: Not enough values for operator"));
                }

                let rhs = output.pop().unwrap().0;
                let lhs = output.pop().unwrap().0;
                match op_stack {
                    Operator::Equals => match lhs {
                        Expression::Variable { id } => self.push_to_output(
                            Expression::Assignment {
                                id,
                                expr: Box::new(rhs),
                            },
                            stack_index,
                            output,
                            stack,
                        ),
                        _ => return Err(String::from("Expr: Left side of assignment has to be variable")),
                    },
                    _ => self.push_to_output(
                        Expression::Operation {
                            op: *op_stack,
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs),
                        },
                        stack_index,
                        output,
                        stack,
                    ),
                }
            }
            TokenType::Id(fn_id)
                if self.known_functions.contains_key(fn_id) && self.known_functions.get(fn_id).unwrap() == &0 =>
            {
                self.push_to_output(
                    Expression::FnCall {
                        fn_id: String::from(fn_id),
                        args: Vec::new(),
                    },
                    stack_index,
                    output,
                    stack,
                )
            }
            t => return Err(format!("Expr: Unexpected token on stack: {:?}", t)),
        };
        return Ok(token);
    }

    fn push_to_output(
        &self,
        element: Expression,
        index: u32,
        output: &mut Vec<(Expression, u32)>,
        stack: &mut Vec<(TokenType, u32)>,
    ) {
        output.push((element, index));

        // Fn calls dont have parenthesis, so we need to check output queue all when it changes
        while !stack.is_empty() {
            let stack_token = stack.pop().unwrap();
            if let (TokenType::Id(fn_id), stack_index) = &stack_token {
                if let Some(arity_u8) = self.known_functions.get(fn_id) {
                    let arity = *arity_u8 as usize;
                    let available_output = output
                        .iter()
                        .skip_while(|(_, idx)| idx < stack_index)
                        .collect::<Vec<_>>()
                        .len();
                    if available_output >= arity {
                        let args = output
                            .drain(output.len() - arity..)
                            .map(|(exp, _)| Box::new(exp))
                            .collect::<Vec<_>>();

                        output.push((
                            Expression::FnCall {
                                fn_id: fn_id.into(),
                                args,
                            },
                            *stack_index,
                        ));
                        continue;
                    }
                }
            }

            stack.push(stack_token);
            break;
        }
    }
}

struct Interpreter {
    lexer: Lexer,
    parser: Parser,
    state: HashMap<String, f32>,
    functions: HashMap<String, (Vec<String>, Expression)>,
}

impl Interpreter {
    fn new() -> Interpreter {
        Self {
            lexer: Lexer::new(),
            parser: Parser::new(),
            state: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    fn input(&mut self, input: &str) -> Result<Option<f32>, String> {
        let tokens = self.lexer.scan(input)?;
        let statement = self.parser.parse(tokens)?;
        let mut initial_state = self.state.clone();
        match statement {
            Statement::FnDef { name, params, body } => {
                if initial_state.contains_key(&name) {
                    return Err(String::from("Eval: Function name already defined as variable"));
                }

                // Can't use same param name twice
                let mut uniq = HashSet::new();
                if !params.iter().all(move |x| uniq.insert(x)) {
                    return Err(String::from("Eval: Function definition has duplicate param names"));
                }

                // Apparently can't use outside variables in function
                if self.check_defined_variables(&body, &params) {
                    self.functions.insert(name, (params, body));
                    Ok(None)
                } else {
                    Err(String::from("Eval: Function definition uses undefined variables"))
                }
            }
            Statement::Expression { expr } => match self.eval(&expr, &mut initial_state) {
                Ok(value) => {
                    self.state.extend(initial_state);
                    Ok(Some(value))
                }
                Err(msg) => Err(msg),
            },
            Statement::Nop => Ok(None),
        }
    }

    fn eval(&self, expr: &Expression, local_state: &mut HashMap<String, f32>) -> Result<f32, String> {
        match expr {
            Expression::Variable { id } => match local_state.get(id) {
                Some(value) => Ok(*value),
                None => Err(format!("Eval: Accessing unknown variable '{}'", id)),
            },
            Expression::Number { value } => Ok(*value),
            Expression::Assignment { id, expr } => {
                let value = self.eval(expr, local_state)?;
                local_state.insert(id.to_owned(), value);
                Ok(value)
            }
            Expression::FnCall { fn_id, args } => {
                let arg_values = args
                    .into_iter()
                    .map(|expr| self.eval(expr, local_state))
                    .collect::<Result<Vec<_>, _>>()?;

                let function = self.functions.get(fn_id);
                match function {
                    Some((params, body)) => {
                        let param_state = params
                            .into_iter()
                            .zip(arg_values)
                            .map(|(p, v)| (String::from(p), v))
                            .collect::<HashMap<_, _>>();
                        let mut func_local_state = local_state
                            .clone()
                            .into_iter()
                            .chain(param_state)
                            .collect::<HashMap<_, _>>();

                        let result = self.eval(body, &mut func_local_state)?;

                        let updated_global_state = func_local_state.into_iter().filter(|(id, _)| !params.contains(id));
                        local_state.extend(updated_global_state);

                        Ok(result)
                    }
                    None => Err(format!("Eval: Accessing unknown function '{}'", fn_id)),
                }
            }
            Expression::Operation { op, lhs, rhs } => {
                let lhs_value = self.eval(lhs, local_state)?;
                let rhs_value = self.eval(rhs, local_state)?;

                match op {
                    Operator::Add => Ok(lhs_value + rhs_value),
                    Operator::Sub => Ok(lhs_value - rhs_value),
                    Operator::Mul => Ok(lhs_value * rhs_value),
                    Operator::Div => Ok(lhs_value / rhs_value),
                    Operator::Mod => Ok(lhs_value % rhs_value),
                    _ => Err(String::from("Eval: Unexpected equals in AST")),
                }
            }
        }
    }

    fn check_defined_variables(&self, expr: &Expression, variables: &Vec<String>) -> bool {
        match expr {
            Expression::Variable { id } => variables.contains(id),
            Expression::Number { .. } => true,
            Expression::Assignment { expr: assign_expr, .. } => self.check_defined_variables(assign_expr, variables),
            Expression::FnCall { args, .. } => args.iter().all(|arg| self.check_defined_variables(arg, variables)),
            Expression::Operation { lhs, rhs, .. } => {
                self.check_defined_variables(lhs, variables) && self.check_defined_variables(rhs, variables)
            }
        }
    }
}
