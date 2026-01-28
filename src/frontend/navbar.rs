use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="fixed top-0 left-0 h-full w-64 bg-gray-800 text-white p-4">
            <div class="h-full flex flex-col">
                <div class="flex flex-col items-center mb-8 p-4">
                    <div class="w-16 h-16 rounded-full overflow-hidden mb-2 border-2 border-gray-600">
                        <img 
                            src="https://ui-avatars.com/api/?name=Eric+Paris&background=75a732&color=fff&size=128&font-size=0.4&bold=true" 
                            alt="EP"
                            class="w-full h-full object-cover"
                        />
                    </div>
                    <div class="text-lg font-bold">"eZoro"</div>
                    <div class="text-sm text-gray-400">"finance for zeros"</div>
                    <div class="text-sm text-gray-400">"plp are amazing"</div>
                    <div class="mt-2 text-sm text-gray-300">"Welcome Eric Paris !"</div>
                </div>
                <ul class="space-y-2">
                    <li><A href="/home" exact=true attr:class="block p-2 rounded hover:bg-gray-700">"Home"</A></li>
                    <li><A href="/portfolio" attr:class="block p-2 rounded hover:bg-gray-700">"Portfolio"</A></li>
                    <li><A href="/search" attr:class="block p-2 rounded hover:bg-gray-700">"Search Assets"</A></li>
                    <li><A href="/wallet" attr:class="block p-2 rounded hover:bg-gray-700">"Wallet"</A></li>                    
                </ul>
            </div>
        </nav>
    }
}
