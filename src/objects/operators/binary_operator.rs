use parsing::frame::Frame;

use objects::object::RcObject;
use objects::result::ObjResult;

pub struct BinaryOperator {
   sigil: &'static str,
   priority: u32,
   is_left_assoc: bool,
   func: fn(RcObject, RcObject, &mut Frame) -> ObjResult
}


mod opers {
   use objects::operators::binary_operator::BinaryOperator;
   use objects::object::RcObject;
   use objects::result::ObjResult;
   use parsing::frame::Frame;

   macro_rules! new_oper {
      ($oper_name:ident, $sigil:expr, $priority:expr, $is_left:ident, $func_name:ident) => {
         fn $func_name(lhs: RcObject, rhs: RcObject, frame: &mut Frame) -> ObjResult {
            lhs.$func_name(rhs, frame)
         }
         pub const $oper_name: BinaryOperator = BinaryOperator {
            sigil: $sigil,
            func: $func_name,
            priority: $priority,
            is_left_assoc: $is_left
         };
      }
   }
   new_oper!(ADD, "+", 12, false, oper_add);
   new_oper!(SUB, "-", 12, false, oper_sub);
   new_oper!(MUL, "*", 11, false, oper_mul);
   new_oper!(DIV, "/", 11, false, oper_div);
   new_oper!(MOD, "%", 11, false, oper_mod);
   new_oper!(POW, "**", 10, true, oper_pow);
}

use std;
derive_impl!(Display; BinaryOperator, sigil);
derive_impl!(Debug; BinaryOperator, "Ob");

impl BinaryOperator {
   pub fn should_exec(&self, other: &BinaryOperator) -> bool {
      (other.is_left_assoc && other.priority >= self.priority) ||
      (!other.is_left_assoc && other.priority > self.priority)
   }

   pub fn exec(&self, frame: &mut Frame) {
      let rhs = frame.pop().expect("bad rhs for operator");
      let lhs = frame.pop().expect("bad lhs for operator");
      let res = ((self.func)(lhs, rhs, frame)).expect("problem with exec of function");
      frame.push(res);
   }
}

use traits::misc::TryFrom;
impl TryFrom for BinaryOperator {
   fn try_from(inp: &str) -> Option<BinaryOperator> {
      match inp {
         "+" => Some(opers::ADD),
         "-" => Some(opers::SUB),
         "*" => Some(opers::MUL),
         "/" => Some(opers::DIV),
         "%" => Some(opers::MOD),
         "**" => Some(opers::POW),
         _ => None
      }
   }
}









