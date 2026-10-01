/// Make me compile!
/// Hint: This time, the compiler won't give you the exact answer.
/// However, it _will_ show you where you should look for possible mistakes!

#[test]
fn calling_call() {
    call_me(3);
}

#[cfg(test)]
fn call_me(num:) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
