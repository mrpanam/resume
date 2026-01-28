use crate::frontend::navbar::Navbar;
use crate::frontend::home::Home;
use crate::frontend::portfolio::Portfolio;
use crate::frontend::assets::Assets;
use crate::frontend::wallet::Wallet;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/resume.css"/>

        // sets the document title
        <Title text="eZoro - Finance App"/>

        // Main app layout with navbar and content
        <Router>
            <div class="min-h-screen">
                <Navbar/>
                <main class="ml-64 p-6">
                    <Routes fallback=|| "Page not found.".into_view()>
                        <Route path=StaticSegment("") view=Home/>
                        <Route path=StaticSegment("home") view=Home/>
                        <Route path=StaticSegment("portfolio") view=Portfolio/>
                        <Route path=StaticSegment("search") view=Assets/>
                        <Route path=StaticSegment("wallet") view=Wallet/>                        
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;
    let my_name = "Eric";

    view! {
        <h1>"Welcome to Leptos! " {my_name}</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        
    }
}
