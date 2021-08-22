extern crate jscjs_sys;

#[cfg(test)]
mod runtime {
    use super::jscjs_sys::runtime as jscjs_sys;

    #[test]
    fn context_value_as_bool() {
        let vm = jscjs_sys::VM::new();
        let context = jscjs_sys::Context::new(&vm);
        let _string = jscjs_sys::String::new("Hello World");
        
        let value = jscjs_sys::Value::with_boolean(&context, false);
        assert_eq!(value.is_boolean(&context), true);
    }
    
    #[test]
    fn context_value_as_number() {
    let vm = jscjs_sys::VM::new();
        let context = jscjs_sys::Context::new(&vm);
        let _string = jscjs_sys::String::new("Hello World");
        
        let value = jscjs_sys::Value::with_number(&context, 42f64);
        assert_eq!(value.to_number(&context).unwrap(), f64::from(42));
    }
    
    #[test]
    fn eval_number() {
        let vm = jscjs_sys::VM::new();
        let context = jscjs_sys::Context::new(&vm);
        let source = url::Url::parse("https://webkit.org").unwrap();
        let object = jscjs_sys::Object::array(&context, &[]).unwrap();
        
        let result = context.evaluate_script("42", &object, source, 0).unwrap();
        assert_eq!(result.is_number(&context), true);
    }
    
    #[test]
    fn eval_string() {
        let vm = jscjs_sys::VM::new();
        let context = jscjs_sys::Context::new(&vm);
        let source = url::Url::parse("https://webkit.org").unwrap();
        let object = jscjs_sys::Object::array(&context, &[]).unwrap();
        
        let result = context.evaluate_script("deadbeef", &object, source, 0);
        assert_eq!(result.is_ok(), false);
    }
    
    #[test]
    fn check_syntax() {
        let vm = jscjs_sys::VM::new();
        let context = jscjs_sys::Context::new(&vm);
        let source = url::Url::parse("https://webkit.org").unwrap();
        
        let result = context.check_syntax("function", source, 0);
        assert_eq!(result.is_ok(), false);
    }
}
