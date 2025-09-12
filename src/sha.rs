use crate::parsing::pars;
use crate::padding::padd;
use crate::schedule::sched;
use crate::compression::compress;

fn sha256(msg: &str) -> String {
    let padding = padd(msg);
    let parsing = pars(padding);
    let schedules = sched(parsing);
    let digest = compress(schedules);

    digest.iter()
        .map(|word| format!("{:08x}", word))
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_string_and_digest() {
        let msg = "";
        let result = sha256(msg);
        let expected = "\
        e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\
        ";

        assert_eq!((result), (expected));
    }

    #[test]
    fn use_one_word_and_digest() {
        let msg = "abc";
        let result = sha256(msg);
        let expected = "\
        ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad\
        ";

        assert_eq!((result), (expected));
    }

    #[test]
    fn expanding_in_second_block_and_digest() {
        let msg = "abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
        let result = sha256(msg);
        let expected = "\
        248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1\
        ";

        assert_eq!((result), (expected));
    }

    #[test]
    fn test_one_million_a_and_digest() {
        let msg = "a".repeat(1_000_000);
        let result = sha256(&msg);
        let expected = "\
        cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0\
        ";

        assert_eq!((result), (expected));
    }
}