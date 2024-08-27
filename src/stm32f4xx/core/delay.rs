

//basic delay function, useful for test
#[allow(dead_code)]
pub fn non_exact_time_delay(delay: u32){
    let mut count = 0;
    while count < delay {
        count+=1;
    }
//some applications may require exact time delay
}
