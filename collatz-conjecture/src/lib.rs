pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut count = 0;
    let mut current = n;

    while current != 1 {
        count += 1;
        if current.is_multiple_of(2) {
            current /= 2;
        } else {
            current = current.checked_mul(3)?.checked_add(1)?;
        }
    }

    Some(count)
}
