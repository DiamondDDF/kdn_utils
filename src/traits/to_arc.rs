use std::sync::{Arc, Mutex};


pub trait ToArc {
    fn arc(self) -> Arc<Self>;
}
//impl ToArcMutex for String {}
impl<T> ToArc for T where T: Sized  {
    fn arc(self) -> Arc<Self>{
        Arc::new(self)
    }
}
#[derive(Debug)]
struct Test {
    num_list: Vec<u32>
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_arc(){
        let test = Test { num_list: vec![1,2,3,4] }.arc();
        println!("{:?}", test.num_list );
    }

}