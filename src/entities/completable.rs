#[derive(Clone)]
pub struct Completable {
    values: Vec<bool>
}

impl Completable {
    pub fn new() -> Self {
        Completable {
            values: vec![false; 9]
        }
    }

    pub fn add_num(&mut self, num: usize) {
        self.values[num - 1] = true;
    }

    pub fn remove_num(&mut self, num: usize) {
        self.values[num - 1] = false;
    }

    pub fn check(&self, num: usize) -> bool {
        !self.values[num - 1]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_add_value() {
        let mut completable = Completable::new();
        completable.add_num(7);
        assert!(!completable.check(7));
    }

    #[test]
    fn must_remove_value() {
        let mut completable = Completable::new();
        completable.add_num(3);
        assert!(!completable.check(3));
        
        completable.remove_num(3);
        assert!(completable.check(3));
    }

    #[test]
    fn must_accept_value_one_to_nine() {
        let mut completable = Completable::new();
        for i in 1..10 {
            completable.add_num(i);
            assert!(!completable.check(i));

            completable.remove_num(i);
            assert!(completable.check(i));
        }
    }

    #[test]
    #[should_panic]
    fn must_panic_if_higher_then_nine() {
        let mut completable = Completable::new();
        completable.add_num(10);
    }

    #[test]
    #[should_panic]
    fn must_panic_if_lower_then_one() {
        let mut completable = Completable::new();
        completable.add_num(0);
    }
}
