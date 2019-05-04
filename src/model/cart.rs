

use super::cart_line::CartLine;
use super::product::Product;

pub struct   Cart<'a>{
lines: Vec<CartLine<'a>>,
item_count : usize,
cart_price : usize,
quantity   : usize
}

impl<'a> Cart<'a> {
//where
//#![feature(type_ascription)]
pub fn addLine(& mut self, product : Product<'a>, quantity : i32   )->Self{
    let mut result = self.lines.iter().find(|&line  | line.product.id == product.id );
    if let Some(result) = result.take() {
        unimplemented!()
        //self.lines.push ( CartLine::new(product: <'_> Product, quantity))
    }else {
        unimplemented!();
        //result.quantity += quantity
    }
    self.recalculate()
}

fn  recalculate(&mut self)-> Self{

    unimplemented!();
    
}
}
