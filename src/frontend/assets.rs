use leptos::prelude::*;
use surrealdb::RecordId;
use crate::backend::db::get_assets;
use crate::frontend::top_assets::TopAssets;

fn record_key(id: &RecordId) -> String {
    String::try_from(id.key().clone()).unwrap_or_else(|_| id.key().to_string())
}

fn risk_class(score: u8) -> &'static str {
    match score {
        1 | 2 => "text-green-700 bg-green-100",
        3 | 4 => "text-lime-700 bg-lime-100",
        5 | 6 => "text-amber-700 bg-amber-100",
        _ => "text-red-700 bg-red-100",
    }
}

fn pct_change_class(pct: Option<f64>) -> &'static str {
    match pct {
        Some(value) if value > 0.0 => "text-green-700",
        Some(value) if value < 0.0 => "text-red-700",
        _ => "text-gray-700",
    }
}

#[component]
pub fn Assets() -> impl IntoView {
    let assets = Resource::new(|| (), |_| get_assets());
    let (search, set_search) = create_signal(String::new());

    view! {
        <div class="p-4 max-w-4xl mx-auto">
            <Suspense fallback=move || {
                view! {
                    <div class="text-center py-8">
                        <div class="inline-block animate-spin rounded-full h-6 w-6 border-t-2 border-b-2 border-blue-500"></div>
                    </div>
                }
            }>
                {move || {
                    assets
                        .get()
                        .map(|result| match result {
                            Ok(items) => {
                                let top_items = items.clone();
                                view! {
                                <TopAssets items=top_items/>
                            }
                            .into_any()
                            }
                            Err(err) => view! {
                                <div class="text-red-600 text-sm p-3 bg-red-50 rounded">
                                    {format!("Error: {}", err)}
                                </div>
                            }.into_any(),
                        })
                }}
            </Suspense>
            <h1 class="text-2xl font-bold text-gray-800 mb-2">"Explore World Markets"</h1>
            <p class="text-gray-600 mb-4">"Explore World Markets"</p>
            <div class="mb-4">
                <input
                    type="text"
                    class="w-full rounded-lg border border-slate-200 px-3 py-2 text-sm text-slate-900 placeholder:text-slate-400 focus:border-teal-500 focus:outline-none focus:ring-1 focus:ring-teal-500"
                    placeholder="Search by symbol or category..."
                    prop:value=search
                    on:input=move |event| set_search.set(event_target_value(&event))
                />
            </div>
            <Suspense fallback=move || {
                view! {
                    <div class="text-center py-8">
                        <div class="inline-block animate-spin rounded-full h-6 w-6 border-t-2 border-b-2 border-blue-500"></div>
                    </div>
                }
            }>
                {move || {
                    assets
                        .get()
                        .map(|result| match result {
                            Ok(items) => {
                                let query = search.get().trim().to_ascii_lowercase();
                                let filtered_items: Vec<_> = if query.is_empty() {
                                    items
                                } else {
                                    items
                                        .into_iter()
                                        .filter(|item| {
                                            let category = record_key(&item.category).to_ascii_lowercase();
                                            item.symbol.to_ascii_lowercase().contains(&query)
                                                || category.contains(&query)
                                        })
                                        .collect()
                                };
                                view! {
                                <div class="overflow-x-auto border rounded-lg">
                                <table class="min-w-full text-sm">
                                    <thead class="bg-teal-500 text-gray-700">
                                        <tr>                                                
                                            <th class="text-left font-medium px-4 py-2">"Symbol"</th>
                                            <th class="text-left font-medium px-4 py-2">"Category"</th>                                                
                                            <th class="text-left font-medium px-4 py-2">"Last Price"</th>
                                            <th class="text-left font-medium px-4 py-2">"% Chg"</th>
                                            <th class="text-left font-medium px-4 py-2">"Risk"</th>
                                        </tr>
                                    </thead>
                                    <tbody class="divide-y">
                                        {filtered_items.into_iter().map(|item| {                                                
                                            let category = record_key(&item.category);
                                            let risk = item.risk.risk_score;
                                            let risk_badge = risk_class(risk);
                                            let last_price = item
                                                .last_price
                                                .map(|price| format!("{:.2}", price))
                                                .unwrap_or_else(|| "-".to_string());
                                            let price_change_class = pct_change_class(item.price_change_pct);
                                            let price_change_pct = item
                                                .price_change_pct
                                                .map(|pct| format!("{:+.2}%", pct))
                                                .unwrap_or_else(|| "-".to_string());
                                            view! {
                                                <tr class="hover:bg-gray-50">
                                                    
                                                    <td class="px-4 py-2 font-medium text-gray-900">{item.symbol}</td>
                                                    <td class="px-4 py-2 text-gray-700">{category}</td>                                                        
                                                    <td class="px-4 py-2 text-gray-700">{last_price}</td>
                                                    <td class=format!("px-4 py-2 {}", price_change_class)>{price_change_pct}</td>
                                                    <td class="px-4 py-2">
                                                        <span class=format!("inline-flex items-center rounded px-2 py-0.5 text-xs font-semibold {}", risk_badge)>
                                                            {risk}
                                                        </span>
                                                    </td>
                                                </tr>
                                            }
                                        }).collect_view()}
                                    </tbody>
                                </table>
                                </div>
                                }
                                .into_any()
                            }
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
