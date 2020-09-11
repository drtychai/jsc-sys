extern crate jscjs_sys;
use crate::jscjs_sys::{Context, Object, VM};

pub fn main() {
    let vm = VM::new();
    let context = Context::new(&vm);
    {
        let source = url::Url::parse("https://webkit.org").unwrap();
        let object = Object::array(&context, &[]).unwrap();
        let result = context.evaluate_script("1", &object, source, 0).unwrap();
        println!("Eval of 1:    {:?}", result);
        assert!(result.is_number(&context));
    }
    {
        let source = url::Url::parse("https://webkit.org").unwrap();
        let object = Object::array(&context, &[]).unwrap();
        let result = context.evaluate_script("1+1", &object, source, 0).unwrap();
        println!("Eval of 1+1:  {:?}", result);
        assert!(result.is_number(&context));
    }
}
