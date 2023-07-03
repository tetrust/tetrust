use crate::lib::components::footer::Footer;
use crate::lib::components::header::Header;
use crate::router::Route;

use stylist::css;
use stylist::yew::Global;
use yew::{function_component, html, Html};
use yew_router::prelude::*;

#[function_component(MainPage)]
pub fn main_page() -> Html {
    let css = css!(
        r##"
        .container {
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
            max-width: none;
        }
    
        .select-form {
            display: flex;
            background-color: #fff;
            align-items: center;
            padding: 40px;
            border-radius: 10px;
            box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
            width: 700px;
            flex-direction: column;
        }
    
        .form-control {
            margin-bottom: 20px;
            display: flex;
		    align-items: center;
        }
    "##
    );

    html! {
        <div>
            <Global css={css} />

            <Header/>

            <main class="container">
                <div class="select-form">
                    <div class="form-control">
                        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 w-40 rounded-full">
                            <Link<Route> to={Route::SinglePlay}>{ "Single Play" }</Link<Route>>
                        </button>
                    </div>
                    <br/>

                    <div class="form-control">
                        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 w-40 rounded-full">
                            <Link<Route> to={Route::MultiPlay}>{ "Multi Play" }</Link<Route>>
                        </button>
                    </div>
                    <br/>

                    <div class="form-control">
                        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 w-40 rounded-full">
                            <Link<Route> to={Route::Setting}>{ "Setting" }</Link<Route>>
                        </button>
                    </div>
                </div>
            </main>

            <Footer/>
        </div>
    }
}
