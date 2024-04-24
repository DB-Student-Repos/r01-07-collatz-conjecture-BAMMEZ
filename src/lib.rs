pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return Some(0); // Edge case: If input is 0, return 0 steps.
    }
    
    let mut steps = 0;
    let mut num = n;
    
    while num != 1 {
        if num % 2 == 0 {
            num /= 2;
        } else {
            num = num * 3 + 1;
        }
        steps += 1;
    }
    
    Some(steps)
}

    
    

