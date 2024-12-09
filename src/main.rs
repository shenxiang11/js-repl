use std::io::Write;

fn main() {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope, Default::default());
    let scope = &mut v8::ContextScope::new(scope, context);

    println!("Javascript REPL - 输入代码并回车执行（Ctrl + C 退出）");

    loop {
        print!("> ");
        std::io::stdout().flush().expect("flush failed");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("read failed");

        let code = v8::String::new(scope, &input).expect("string conversion failed");
        let script = v8::Script::compile(scope, code, None).unwrap();
        let result = script.run(scope).expect("script execution failed");
        let result = result.to_string(scope).expect("string conversion failed");

        println!("{}", result.to_rust_string_lossy(scope));
    }
}
