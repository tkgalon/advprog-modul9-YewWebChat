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

    let avatar_url = "https://avatars.dicebear.com/api/adventurer-neutral/default.svg".to_string();

    html! {
       <div class="bg-gradient-to-r from-purple-600 to-blue-500 flex w-screen h-screen">
            <div class="container mx-auto flex flex-col justify-center items-center">
                <div class="bg-white p-8 rounded-lg shadow-lg w-96">
                    <h1 class="text-3xl font-bold text-center mb-2 text-purple-700">{"Welcome to YewChat!"}</h1>
                    <p class="text-gray-600 text-center mb-6">{"Connect with friends in real-time using Rust and WebAssembly"}</p>
                    
                    <div class="flex justify-center mb-6">
                        <div class="border-4 border-purple-200 rounded-full p-1">
                            <img src={avatar_url} class="w-24 h-24 rounded-full" alt="avatar preview"/>
                        </div>
                    </div>
                    
                    <div class="text-center mb-6">
                        <p class="text-sm text-gray-500">{"Preview your avatar above"}</p>
                    </div>
                    
                    <form class="flex flex-col">
                        <input 
                            {oninput} 
                            class="rounded-lg p-4 border text-gray-800 border-gray-300 bg-white mb-4 focus:outline-none focus:ring-2 focus:ring-purple-400" 
                            placeholder="Choose a username" 
                        />
                        <Link<Route> to={Route::Chat}>
                            <button 
                                {onclick} 
                                disabled={username.len()<1} 
                                class="w-full px-8 rounded-lg bg-gradient-to-r from-purple-600 to-blue-500 hover:from-purple-700 hover:to-blue-600 text-white font-bold p-4 uppercase transition duration-300 ease-in-out transform hover:-translate-y-1"
                            >
                                {"Start Chatting!"}
                            </button>
                        </Link<Route>>
                    </form>
                </div>
            </div>
        </div>
    }
}