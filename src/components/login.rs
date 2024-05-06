use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };
    
    html! {
        <div class="bg-gray-800 flex w-screen">
            <div class="container mx-auto flex gap-5 justify-center items-center">
                <div class="flex flex-col gap-2 justify-start text-white"> 
                    <div class="text-6xl font-bold text-violet-500"> {"YewChat"} </div>
                    <p class="text-3xl mb-4 font-bold"> {"Yew and I, \nOne Chat at a Time"} </p>
                    <p class="text-xl font-bold"> {"How should we recognize you?"} </p>
                    <form class="flex">
                        <input {oninput} class="rounded-l-lg p-4 border-t mr-0 border-b border-l text-gray-800 border-gray-200 bg-white" placeholder="Username"/>
                        <Link<Route> to={Route::Chat}> <button {onclick} disabled={username.len()<1} class="px-8 rounded-r-lg bg-violet-600	  text-white font-bold p-4 uppercase border-violet-600 border-t border-b border-r" >{"Go Chatting!"}</button></Link<Route>>
                    </form>
                </div>
                <div>
                    <img src="https://i.pinimg.com/originals/0d/50/1e/0d501e12d8c8c68423e5b1a1cb319879.png" alt="Chat" width="300px" height="auto"/>
                </div>
            </div>
        </div>
    }
}