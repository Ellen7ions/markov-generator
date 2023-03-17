use crate::typo::{Character, Context};

pub struct Slicer<'a> {
    contents: &'a Vec<u8>,
    ptr: usize,
    context: Context,
}

impl<'a> Slicer<'a> {
    pub fn new(contents: &'a Vec<u8>) -> Self {
        Self {
            contents: contents,
            ptr: 0,
            context: 0,
        }
    }
}

impl<'a> Iterator for Slicer<'a> {
    type Item = (Context, Character);

    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr >= self.contents.len() {
            return None;
        }
        let res = Some((self.context, self.contents[self.ptr]));
        self.context =  (self.context << 8) | (self.contents[self.ptr] as Context);
        self.ptr += 1;
        res
    }
}
