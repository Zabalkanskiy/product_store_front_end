
pub struct Product<'a>{
id: Option<usize>,
name: Option<&'a str>,
category:Option<&'a str>,
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

pub struct ProductRepositiry<'a>{
products : Box<Vec<Product<'a>>>,
categories : Vec<String>,
}

impl<'a> ProductRepositiry<'a>{
pub fn getProducts(&self, category: Option< &str>)->Box<Vec<Product>>{
    let result : Vec<_>  = self.products.into_iter().filter(|&p : &Product|{
        if  p.category == None || p.category == category {
            true
        } else {
            false
        } 

    }).collect();

    Box::new(result)
} 

pub fn getProduct(&self, id: Option<usize>) -> Option<Product> {
    let res: Option<Product> = self.products.into_iter().find(|&p| p.id == id);
    res
}

pub fn getCategories(&self) -> Vec<String>{
    self.categories
}
}

pub struct   Cart<'a>{
lines: Vec<CartLine<'a>>,
item_count : usize,
cart_price : usize,
quantity   : usize
}

impl<'a> Cart<'a> {
//where

pub fn addLine(&self, product : Product, quantity : i32   )->Self{
    let result = self.lines.iter().find(|&line  | line.product.id == product.id );
    if let Some(result) = result.take() {
        self.lines.push ( CartLine <'a> {product<'a>, quantity})
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

pub struct CartLine<'a>{
product : Product<'a>,
quantity : i32
}

fn filter_one(input: &[u8]) -> impl Iterator<Item = &u8> {
input.iter().filter(|&&x| x == 1)
}