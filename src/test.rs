
#[cfg(test)]
mod tests {
    use crate::blockchain::*;

    #[test]
    fn test_blockchain() {
        let mut b = Blockchain::new().unwrap();
        b.add_block("data".to_string());
        b.add_block("data2".to_string());
        b.add_block("data3".to_string());
        dbg!(&b);

        for item in b.iter(){
            println!("item {:?}",item)
        }
    }
}
