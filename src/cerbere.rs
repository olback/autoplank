pub struct Cerbere {
    vec: Vec<String>
}

impl Cerbere {

    pub fn from<S: Into<String>>(val: S) -> Self {

        let input = val.into();
        let res = &input[1..][..input.len() - 2];
        let vec: Vec<String> = res.split(", ").map(|v| String::from(v)).collect();

        Self {
            vec: vec
        }

    }

    pub fn add<S: Into<String>>(&mut self, item: S) -> bool {

        let item_str = item.into();

        if !self.vec.contains(&item_str) {
            self.vec.push(item_str);
            return true;
        }

        false

    }

    pub fn remove<S: Into<String>>(&mut self, item: S) -> bool {

        let item_str = item.into();

        // Nightly
        // self.vec.remove_item(&item_str).is_some()

        if self.vec.contains(&item_str) {
            let index = self.vec.iter().position(|v| v == &item_str).unwrap();
            self.vec.remove(index);
            return true;
        }

        false

    }

    pub fn to_string(&self) -> String {

        format!("[{}]", self.vec.join(", "))

    }

}
