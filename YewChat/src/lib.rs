#![recursion_limit = "512"]

mod components;
mod services;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use components::chat::Chat;
use components::login::Login;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Login,
    #[at("/chat")]
    Chat,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub type User = Rc<UserInner>;

pub type DarkMode = bool;

#[derive(Debug, PartialEq)]
pub struct UserInner {
    pub username: RefCell<String>,
}

// toggle html class for dark mode
use web_sys::window;
#[function_component(Main)]
fn main() -> Html {
    let ctx = use_state(|| {
        Rc::new(UserInner {
            username: RefCell::new("initial".into()),
        })
    });
    let dark_mode = use_state(|| false);
    
    let toggle_dark = {
        let dark_mode = dark_mode.clone();
        Callback::from(move |_| {
            dark_mode.set(!*dark_mode);
        })
    };

    let is_dark = *dark_mode;
    
    html! {
        <ContextProvider<User> context={(*ctx).clone()}>
        <ContextProvider<DarkMode> context={is_dark}>
            <div class={if is_dark { "dark" } else { "" }}>
                <button onclick={toggle_dark} class="fixed top-2 right-2 z-50 bg-gray-200 dark:bg-gray-700 rounded px-3 py-1">
                    { if is_dark { "☀️" } else { "🌙" } }
                </button>
                <BrowserRouter>
                    <div class="flex w-screen h-screen">
                        <Switch<Route> render={Switch::render(switch)}/>
                    </div>
                </BrowserRouter>
            </div>
        </ContextProvider<DarkMode>>
        </ContextProvider<User>>
    }
}

fn switch(selected_route: &Route) -> Html {
    match selected_route {
        Route::Login => html! {<Login />},
        Route::Chat => html! {<Chat/>},
        Route::NotFound => html! {<h1>{"404 baby"}</h1>},
    }
}


#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Main>();
    Ok(())
}

//recommit