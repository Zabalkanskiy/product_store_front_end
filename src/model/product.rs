#[derive(Copy, Clone)]
pub struct Product<'a>{
pub id: Option<usize>,
name: Option<&'a str>,
pub category:Option<&'a str>,
descriptions: Option<&'a str>,
price: Option<usize>,
}

impl<'a> Product<'a>{
pub fn new()-> Self{
    Product{
        id: None,
        name: None,
        category: None,
        descriptions: None,
        price: None,
    }
}
}