pub struct RleCodec;

impl RleCodec {
    pub fn encode(input: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(input.len());
        let mut i = 0;
        while i < input.len() {
            let b = input[i];
            let mut run = 1usize;
            while i + run < input.len() && input[i + run] == b && run < 255 {
                run += 1;
            }
            if run >= 3 {
                out.push(0xFF);
                out.push(b);
                out.push(run as u8);
                i += run;
            } else {
                for _ in 0..run {
                    out.push(b);
                }
                i += run;
            }
        }
        out
    }

    pub fn decode(input: &[u8]) -> Result<Vec<u8>, crate::error::DecodeError> {
        let mut out = Vec::new();
        let mut i = 0;
        while i < input.len() {
            if input[i] == 0xFF && i + 2 < input.len() {
                let b = input[i + 1];
                let run = input[i + 2] as usize;
                out.extend(std::iter::repeat(b).take(run));
                i += 3;
            } else {
                out.push(input[i]);
                i += 1;
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roundtrip() {
        let data = b"aaabbbccc";
        let enc = RleCodec::encode(data);
        let dec = RleCodec::decode(&enc).unwrap();
        assert_eq!(dec, data);
    }
}
