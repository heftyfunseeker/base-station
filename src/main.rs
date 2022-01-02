use yew::prelude::*;

mod anthem;
use crate::anthem::AnthemIP;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        html! {
            <div class="toggle-container">
            <div> {"AVR Off"} </div>
             <label class="switch">
               <input type="checkbox" onclick={
                   ctx.link().callback(|_| {
                       Msg::AddOne
                   })
               }/>
               <span class="slider round"></span>
             </label>
            <div> {"AVR On"} </div>
            </div>
        }
    }
}

fn main() {
    //let av = AnthemIP::new("ws://192.168.0.28:8080");
    yew::start_app::<Model>();
}