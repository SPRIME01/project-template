use boa_engine::{Context, Source};

fn main() {
    // Constant, trusted ECMAScript only. Do NOT accept arbitrary scripts as
    // arguments without execution/time/memory and authority controls.
    let mut context = Context::default();
    let result = context
        .eval(Source::from_bytes("1 + 2"))
        .expect("the fixed JavaScript expression should evaluate");
    assert_eq!(result.as_number(), Some(3.0));
    println!("Boa Rust JavaScript engine PASS: 1 + 2 = 3");
}

#[cfg(test)]
mod tests {
    use boa_engine::{Context, Source};

    #[test]
    fn execution_without_v8() {
        let mut engine = Context::default();
        let value = engine.eval(Source::from_bytes("Math.max(2, 5)")).unwrap();
        assert_eq!(value.as_number(), Some(5.0));
    }
}
