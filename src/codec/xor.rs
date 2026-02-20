pub fn xor_checksum(data: &[u8]) -> u32 {
    data.iter().fold(0u32, |acc, &b| acc ^ u32::from(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_payload() {
        assert_eq!(xor_checksum(b""), 0);
    }
}
