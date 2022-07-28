fn main() {
    let mut count = 0;
    let mut inc = move || {
        count += 1;
        println!("`count`: {}", count);
    };
    inc();
    let _reborrow = &count; 
    inc();
    let _count_reborrowed = &mut count; 
    assert_eq!(count, 0);
}