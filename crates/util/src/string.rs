

pub trait StringExt {
    fn sub_str(&self, start: i64) -> &str;
}

impl StringExt for String {
    fn sub_str(&self, start: i64) -> &str {

        if start == 0 {
            return &self;
        }

        let len = self.len();
        let _start =  if start < 0 { (start * -1) as usize } else { start as usize };
        if _start >= len {
            return &self;
        }


        if start < 0 {
            &self[_start..]
        } else {
            &self[.._start]
        }
    }
}
