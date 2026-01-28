use leptos::prelude::*;
use crate::backend::db::get_wallet;

#[component]
pub fn Wallet() -> impl IntoView {
    let wallet_entries = Resource::new(|| (), |_| get_wallet());
    const EUR_USD_RATE: f64 = 1.09;

    view! {
        <div class="p-4 max-w-3xl mx-auto">
            <h1 class="text-2xl font-bold text-gray-800 mb-2">"Wallet"</h1>
            <p class="text-gray-600 mb-6">"Transactions"</p>

            <Suspense fallback=move || {
                view! {
                    <div class="text-center py-8">
                        <div class="inline-block animate-spin rounded-full h-6 w-6 border-t-2 border-b-2 border-blue-500"></div>
                    </div>
                }
            }>
                {move || {
                    wallet_entries
                        .get()
                        .map(|result| match result {
                            Ok(items) => {
                                let total_eur: f64 = items
                                    .iter()
                                    .map(|item| {
                                        let amount = item.amount as f64;
                                        if item.ccy.eq_ignore_ascii_case("EUR") {
                                            amount
                                        } else if item.ccy.eq_ignore_ascii_case("USD") {
                                            amount / EUR_USD_RATE
                                        } else {
                                            amount
                                        }
                                    })
                                    .sum();

                                view! {
                                    <ul class="space-y-3">
                                        {items.into_iter().map(|item| {
                                            let amount = item.amount as f64;
                                            let label = if amount < 0.0 { "Debit" } else { "Credit" };
                                            let amount_abs = amount.abs();
                                            let label_class = if amount < 0.0 { "text-red-600" } else { "text-green-600" };
                                            let tx_date = item
                                                .tx_date
                                                .into_inner_ref()
                                                .format("%Y-%m-%d %H:%M UTC")
                                                .to_string();

                                            view! {
                                                <li class="p-4 border rounded-lg flex items-center justify-between gap-4">
                                                    <div class="flex flex-col">
                                                        <span class="text-sm text-gray-500">"Date"</span>
                                                        <span class={format!("font-medium {}", label_class)}>{label}</span>
                                                    </div>
                                                    <div class="text-right">
                                                        <div class="text-sm text-gray-500">{tx_date}</div>
                                                        <div class="text-lg font-semibold text-gray-900">
                                                            {format!("{:.2} {}", amount_abs, item.ccy)}
                                                        </div>
                                                    </div>
                                                </li>
                                            }
                                        }).collect_view()}
                                    </ul>
                                    <div class="mt-6 p-4 border rounded-lg flex items-center justify-between">
                                        <div class="text-sm text-gray-600">
                                            {format!("Rate used: EUR/USD {:.4}", EUR_USD_RATE)}
                                        </div>
                                        <div class="text-lg font-semibold text-gray-900">
                                            {format!("Total: {:.2} EUR", total_eur)}
                                        </div>
                                    </div>
                                }
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
