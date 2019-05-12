

use crate::model::*;

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};
use yew::format::Json;
use yew::services::{DialogService, StorageService};
use yew::services::storage::Area;

#[derive(Debug)]
pub enum Scene {
    MarketList,
    CartList(Client),
    Settings,
    ClientsList,
}

//Фамиоия имя отчество клиента
#[derive(Debug)]
pub enum Msg {
    SwitchTo(Scene),
    AddToCart,
    UpdateFirstName(String),
    UpdateLastName(String),
    UpdateDescription(String),
    Clear,
}

#[derive(Debug)]
pub struct Client;

impl Client{

    fn clients(){
        unimplemented!();
    }
}

pub struct Model{
    scene : Scene,
    database : Client,
    storage: (),
    dialog: DialogService,
    
}
impl Component for Model{
    type Message = Msg;
    type Properties = ();


    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        unimplemented!();
    }        

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let mut new_scene : Option<Scene> = None;
        match self.scene {
             Scene::MarketList => {
                match msg {
                    Msg::SwitchTo(Scene::CartList(client)) => {
                        new_scene = Some(Scene::CartList(client));
                    }
                    Msg::SwitchTo(Scene::Settings) => {
                        new_scene = Some(Scene::Settings);
                    }
                    unexpected => {
                        panic!("Unexpected message when clients list shown: {:?}", unexpected);
                    }
                }
            },

            Scene::Settings => {
                match msg {
                    Msg::Clear => {
                        let ok = {
                            self.dialog.confirm("Do you really want to clear the data?")
                        };
                        if ok {
                            unimplemented!();
                            //self.database.clients.clear();
                            //self.storage.remove(KEY);
                        }
                    }
                    Msg::SwitchTo(Scene::ClientsList) => {
                      new_scene = Some(Scene::ClientsList);
  },
                
                    unexpected => {
                        panic!("Unexpected message for settings scene: {:?}", unexpected);
                    }
                }

            },
                Scene::CartList(_)=>{
                        unimplemented!()
                    },
                Scene::ClientsList =>{
                    unimplemented!()
                }

           
        }
         if let Some(new_scene) = new_scene.take() {
                self.scene = new_scene; 
            }

        true

    }

}
impl Renderable<Model> for Model  {
    fn view(&self)-> Html<Self>{
        match &self.scene {
            Scene::MarketList =>{
             unimplemented!();   
            },
            Scene::ClientsList => {
                unimplemented!();
            },
            Scene::CartList(client) => {
                unimplemented!();
            },
            Scene::Settings =>{
                unimplemented!();
            }
        }
    }
}
