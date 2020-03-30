#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod geno {

    pub fn add(id1: i32, id2: i32, id3: i32) {
        id1 + id2 + id3;
    }

    pub fn randnum() { rand::random::<i32>(); }

}




