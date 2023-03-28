pub struct TestTools;

impl TestTools {
    pub fn random_strings(num: usize, length: usize) -> Vec<String> {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
        let mut rng = rand::thread_rng();
        let mut strings = Vec::new();

        for _ in 0..num {
            let s: String = (0..length)
                .map(|_| {
                    let idx = rng.gen_range(0..CHARSET.len());
                    // This is safe because `idx` is in range of `CHARSET`
                    char::from(unsafe { *CHARSET.get_unchecked(idx) })
                })
                .collect();
            strings.push(s);
        }

        strings
    }
}
