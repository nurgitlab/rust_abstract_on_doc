pub struct RandomNumber {
    pub value: u8,
}

impl RandomNumber {
    pub fn new (value: u8) -> Self {
        super::super::shared(); // Call the shared function from main.rs

        return Self {
            value
        }
    }
}