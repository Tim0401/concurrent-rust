struct Statement;

impl Statement {
    fn is() -> bool {
        !Self::is()
    }
}

fn main() {
    assert!(Statement::is())
}
