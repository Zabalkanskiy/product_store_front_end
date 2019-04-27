
pub struct Product{
    id: Option<usize>,
    name: Option<&str>,
    category:Option<&str>,
    descriptions: Option<&str>,
    price: Option<usize>,
}

impl Product{
    pub fn new()-> Self{
        Product{
            id: None,
            name: None,
            category: None,
            descriptions: None,
            price: None
        }
    }
}

pub struct ProductRepositiry{
    products : Box<Vec<Product>>,
    categories : Vec<String>,
}

impl ProductRepositiry{
    pub fn getProducts(&self, category: Option< &str>)->Box<Vec<Product>>{
       let result  = self.products.into_iter().filter(|&p : &Product|{
          if  p.category == None || p.category == category {
              true
          } else {
              false
          } 

        });

        result
    } 

    pub fn getProduct(&self, id: Option<usize>) ->  Product {
      let res = self.products.into_iter().find(|&p|{ if p.id == id{true}else {false}})
    }

    pub fn getCategories(&self) -> Vec<String>{
        self.categories
    }
}

pub struct   Cart{
    lines: Vec<CartLine>,
    item_count : usize,
    cart_price : usize,

}

impl Cart {

    pub fn addLine(&self, product : Product, quantity : i32   ){
      let result = self.lines.find(|line|-> Cartline {line.product.id == product.id} );
        if result != None {
            result.quantity += quantity
        }else {
            self.lines.push ( CartLine{product, quantity})
        }
        self.recalculate
    }

  fn  recalculate(&self){

        unimplemented!();
        
    }
}

pub struct CartLine{
   product : Product,
   quantity : i32
}

