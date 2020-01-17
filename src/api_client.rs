pub struct Kintone<'a> {
    base_url: &'a str
}

impl<'a> Kintone<'a> {
    pub fn new(base_url: &'a str) -> Kintone {
        Kintone {
            base_url
        }
    }
    pub fn get_base_url(&self) -> &str {
        self.base_url
    }
}