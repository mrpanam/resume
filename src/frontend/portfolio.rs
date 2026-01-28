use leptos::prelude::*;
use crate::backend::db::get_categories;

#[component]
pub fn Portfolio() -> impl IntoView {
    let categories = Resource::new(|| (), |_| get_categories());

    view! {
        <div class="p-4 max-w-4xl mx-auto">
            <h1 class="text-2xl font-bold text-gray-800 mb-2">"Portfolio"</h1>
            <p class="text-gray-600 mb-6">"Browse my work by category"</p>
            
            <Suspense fallback=move || 
                view! { 
                    <div class="text-center py-8">
                        <div class="inline-block animate-spin rounded-full h-6 w-6 border-t-2 border-b-2 border-blue-500"></div>
                    </div>
                }>
                {move || {
                    categories
                        .get()
                        .map(|result| match result {
                            Ok(items) => view! {
                                <div class="grid gap-4 sm:grid-cols-2">
                                    {items.into_iter().map(|item| view! {
                                        <div class="p-4 border rounded-lg hover:bg-gray-50 transition-colors">
                                            <div class="flex items-center gap-3">
                                                <div class="flex-shrink-0 w-8 h-8 rounded bg-blue-100 text-blue-600 flex items-center justify-center text-sm font-medium">
                                                    {item.name.chars().next().unwrap_or(' ')}
                                                </div>
                                                <h3 class="font-medium text-gray-900">{item.name}</h3>
                                            </div>
                                            <p class="mt-2 text-sm text-gray-600">
                                                {item.description}
                                            </p>
                                        </div>
                                    }).collect_view()}
                                </div>
                            }
                            .into_any(),
                            Err(err) => view! { 
                                <div class="text-red-600 text-sm p-3 bg-red-50 rounded">
                                    {format!("Error: {}", err)}
                                </div>
                            }.into_any(),
                        })
                }}
            </Suspense>
        </div>
    }
}
