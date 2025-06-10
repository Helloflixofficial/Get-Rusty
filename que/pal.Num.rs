impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut x = x;
        let mut rev = 0;

        while x > rev {
            rev = rev * 10 + x % 10;
            x /= 10;
        }

        x == rev || x == rev / 10
    }
}
