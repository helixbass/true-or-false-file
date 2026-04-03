fn main() {
    println!("{}", parser_truefalse("".as_bytes()));
}


fn parser_truefalse(file_content: &[u8]) -> bool {
    if file_content.len() < 1 {
        return false
    } else {
        return true
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3_long() {
        let pars_candy = parser_truefalse("abc".as_bytes());
        assert_eq!(pars_candy, true);
    }
}