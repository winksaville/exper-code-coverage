use exper_code_coverage::{
    if_short_circuit_and::if_short_circuit_and, short_circuit_and::short_circuit_and,
};

fn main() {
    println!("Evaluates: a < b && c < d using:\n\
              \x20 where `if_short_circuit_and(a, b, c, d) -> bool` uses if statement\n\
              \x20 and `short_circuit_and(a, b, c, d) -> bool` uses the single short-circuit expression");
    let a = 10;
    let b = 20;
    let c = 30;
    let d = 40;

    let r = if_short_circuit_and(b, a, c, d);
    println!("if_short_circuit_and({b}, {a}, {c}, {d})={r}");
    let r = if_short_circuit_and(a, b, d, c);
    println!("if_short_circuit_and({a}, {b}, {d}, {c})={r}");
    let r = if_short_circuit_and(a, b, c, d);
    println!("if_short_circuit_and({a}, {b}, {c}, {d})={r}");

    let r = short_circuit_and(b, a, c, d);
    println!("short_circuit_and({b}, {a}, {c}, {d})={r}");
    let r = short_circuit_and(a, b, d, c);
    println!("short_circuit_and({a}, {b}, {d}, {c})={r}");
    let r = short_circuit_and(a, b, c, d);
    println!("short_circuit_and({a}, {b}, {c}, {d})={r}");
}
