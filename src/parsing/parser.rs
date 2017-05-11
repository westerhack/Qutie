use parsing::stream::Stream;
use parsing::frame::Frame;
use parsing::token::Token;
use parsing::operator::Operator;
use obj::traits::ToRc;
use std::rc::Rc;
use obj::objects::number::Number;
use obj::objects::object::Object;
use obj::objects::block::LParen;


pub fn parse<'a>(stream: &'a mut Stream<'a>) {
   let ref mut frame = Frame::new();
   while !stream.is_empty() {
      parse_expr(next_expr(stream), frame);
   }
   println!("frame: {:?}", frame);
}

fn next_expr(stream: &mut Stream) -> Vec<Token> {
   let mut ret = vec![];
   while let Some(token) = stream.next() {
      match token {
         Token::LineTerminator(end) => { ret.push(Token::LineTerminator(end)); break },
         Token::Unknown(chr) => panic!("Unknown character: {:?}", chr),
         token @ _ => ret.push(token)
      }
   }
   ret
}

fn handle_assignment(mut tokens: Vec<Token>, frame: &mut Frame) {
   assert!(2 < tokens.len(), "need at least 3 operands for assignment!");
   let identifier = 
      match tokens.remove(0) {
         Token::Identifier(identifier) => identifier,
         other @ _ => panic!("can only assign to identifiers not {:?}", other)
      };
   let assign_type = 
      match tokens.remove(0) {
         Token::Assignment(assign_type) => assign_type,
         other @ _ => unreachable!("The second thing should always be an assignment value, not {:?}!", other)
      };
   parse_expr(tokens, frame);
   let val = frame.pop().expect("cant set a key to nothing!");
   frame.push(val.clone());
   frame.set(identifier, val);
}

fn parse_expr(mut tokens: Vec<Token>, frame: &mut Frame) {
   if tokens.is_empty() { return }

   let is_assignment = 
      2 < tokens.len() && 
      match tokens.get(1).expect("no assignment!") {
         &Token::Assignment(_) => true,
         _ => false
      };

   if is_assignment {
      handle_assignment(tokens, frame);
      return
   }

   let mut oper_stack = Vec::<Operator>::new();
   for token in tokens {
      match token {
         Token::Identifier(id)        => 
            {
               if let Some(val) = frame.get(&id) {
                  if false /*val is a function */ {
                     /* val.call next argument in tokens */ panic!()
                  } else {
                     frame.push(val)
                  }
               } else {
                  panic!("unknown identifier: {:?}", id)
               }
            },
         Token::Number(num)           => frame.push(Number::from(num).to_rc()),
         Token::Operator(oper)        => 
         {
            while !oper_stack.is_empty() {
               if !oper_stack.last().unwrap().should_exec(&oper) {
                  break
               }
               oper_stack.pop().unwrap().exec(frame);
            }
            oper_stack.push(oper)
         }
         Token::Text(quote, body)     => unimplemented!(),
         Token::Path(path)            => unimplemented!(),
         Token::Block((lp, rp), body) => 
            match lp {
               LParen::Round => parse_expr(body, frame),
               LParen::Square => panic!("what to do with square?"),
               LParen::Curly => panic!("What to do with curly?"),
            },
         Token::Unknown(_)        => unreachable!(),
         Token::LineTerminator(_) =>
            {
               while let Some(oper) = oper_stack.pop() {
                  oper.exec(frame);
               }
               frame.pop();
               return
            },
         Token::Assignment(_)     => unreachable!(),
         Token::RParen(_)         => unreachable!(), 
      }
   };
   while let Some(oper) = oper_stack.pop() {
      oper.exec(frame);
   }
}






