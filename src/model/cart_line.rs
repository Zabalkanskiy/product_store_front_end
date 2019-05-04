use super::product::Product;

pub struct CartLine<'a>{
pub product : Product<'a>,
quantity : i32
}

impl<'a> CartLine<'a> {
    pub fn new (product: Product, quantity : i32)->CartLine {
       CartLine{
            product: product,
            quantity
        }
        
    }
}