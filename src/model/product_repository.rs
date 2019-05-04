

use super::product::Product;

pub struct ProductRepository<'a>{
products : Box<Vec<Product<'a>>>,
categories : Vec<String>,
}

impl<'a> ProductRepository<'a>{
pub fn getProducts(&self, category: Option< &str>)->Box<Vec<Product>>{
    let result : Vec<_>  = self.products.clone().into_iter().filter(move |&p : &Product|{
        if  p.category == None || p.category == category {
            true
        } else {
            false
        } 

    }).collect();

    Box::new(result)
} 

pub fn getProduct(&self, id: Option<usize>) -> Option<Product> {
    let res: Option<Product> = self.products.clone().into_iter().find(|&p| p.id == id);
    res
}


pub fn getCategories(&self) -> Vec<String>{
    self.categories.clone()
}
}

