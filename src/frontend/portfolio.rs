use leptos::prelude::*;
use surrealdb::RecordId;
use crate::backend::db::get_trades;

fn record_key(id: &RecordId) -> String {
    String::try_from(id.key().clone()).unwrap_or_else(|_| id.key().to_string())
}

fn status_badge(status: &str) -> &'static str {
    match status.to_ascii_lowercase().as_str() {
        "open" => "text-emerald-700 bg-emerald-100",
        "closed" => "text-slate-700 bg-slate-200",
        "pending" => "text-amber-700 bg-amber-100",
        _ => "text-slate-700 bg-slate-100",
    }
}

#[component]
pub fn Portfolio() -> impl IntoView {
    let trades = Resource::new(|| (), |_| get_trades());

    view! {
        <div class="p-4 max-w-4xl mx-auto">
            <h1 class="text-2xl font-bold text-gray-800 mb-2">"Portfolio"</h1>
            <p class="text-gray-600 mb-6">"Recent trades"</p>

            <Suspense fallback=move || {
                view! {
                    <div class="text-center py-8">
                        <div class="inline-block animate-spin rounded-full h-6 w-6 border-t-2 border-b-2 border-blue-500"></div>
                    </div>
                }
            }>
                {move || {
                    trades
                        .get()
                        .map(|result| match result {
                            Ok(items) => view! {
                                <div class="overflow-x-auto border rounded-lg">
                                    <table class="min-w-full text-sm">
                                        <thead class="bg-slate-100 text-gray-700">
                                            <tr>
                                                <th class="text-left font-medium px-4 py-2">"Asset"</th>
                                                <th class="text-left font-medium px-4 py-2">"Price"</th>
                                                <th class="text-left font-medium px-4 py-2">"Qty"</th>
                                                <th class="text-left font-medium px-4 py-2">"Status"</th>
                                                <th class="text-left font-medium px-4 py-2">"Trade Date"</th>
                                            </tr>
                                        </thead>
                                        <tbody class="divide-y">
                                            {items.into_iter().map(|item| {
                                                let asset = record_key(&item.asset);
                                                let price = format!("{:.2}", item.price);
                                                let qty = if item.qty.fract() == 0.0 {
                                                    format!("{:.0}", item.qty)
                                                } else {
                                                    format!("{:.2}", item.qty)
                                                };
                                                let status_class = status_badge(&item.status);
                                                let trade_date = item
                                                    .trade_date
                                                    .into_inner_ref()
                                                    .format("%Y-%m-%d %H:%M UTC")
                                                    .to_string();
                                                view! {
                                                    <tr class="hover:bg-gray-50">
                                                        <td class="px-4 py-2 font-medium text-gray-900">{asset}</td>
                                                        <td class="px-4 py-2 text-gray-700">{price}</td>
                                                        <td class="px-4 py-2 text-gray-700">{qty}</td>
                                                        <td class="px-4 py-2">
                                                            <span class=format!("inline-flex items-center rounded px-2 py-0.5 text-xs font-semibold {}", status_class)>
                                                                {item.status}
                                                            </span>
                                                        </td>
                                                        <td class="px-4 py-2 text-gray-700">{trade_date}</td>
                                                    </tr>
                                                }
                                            }).collect_view()}
                                        </tbody>
                                    </table>
                                </div>
                            }
                            .into_any(),
                            Err(err) => view! {
                                <div class="text-red-600 text-sm p-3 bg-red-50 rounded">
                                    {format!("Error: {}", err)}
                                </div>
                            }
                            .into_any(),
                        })
                }}
            </Suspense>
        </div>
    }
}
