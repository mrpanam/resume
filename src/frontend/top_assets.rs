use leptos::prelude::*;
use surrealdb::RecordId;

use crate::backend::model::AssetWithPrice;

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

fn pct_sort_value(pct: Option<f64>) -> f64 {
    match pct {
        Some(value) if value.is_finite() => value,
        _ => f64::NEG_INFINITY,
    }
}

fn category_bg_class(category: &str) -> &'static str {
    match category.trim().to_ascii_lowercase().as_str() {
        "bonds" => "bg-slate-200/80",
        "commodities" => "bg-amber-200/80",
        "crypto" => "bg-orange-200/80",
        "forex" => "bg-sky-200/80",
        "indice" => "bg-rose-200/80",
        "stocks" => "bg-teal-200/80",
        _ => "bg-gray-200/80",
    }
}

#[component]
pub fn TopAssets(items: Vec<AssetWithPrice>) -> impl IntoView {
    let mut top_assets: Vec<&AssetWithPrice> = items.iter().collect();
    top_assets.sort_by(|a, b| {
        pct_sort_value(b.price_change_pct)
            .partial_cmp(&pct_sort_value(a.price_change_pct))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    top_assets.truncate(6);

    if top_assets.is_empty() {
        return view! {
            <div class="mb-6 rounded-xl border border-slate-200 bg-white p-4 text-sm text-slate-500">
                "No performance data available yet."
            </div>
        }
        .into_any();
    }

    view! {
        <section class="mb-6">
            <div class="flex items-center justify-between mb-3">
                <div>
                    <h2 class="text-lg font-semibold text-slate-900">"Top Performers"</h2>
                    <p class="text-sm text-slate-500">"Best momentum by recent price change."</p>
                </div>
            </div>
            <div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
                {top_assets
                    .into_iter()
                    .map(|item| {
                        let category = record_key(&item.category);
                        let card_bg = category_bg_class(&category);
                        let risk_badge = risk_class(item.risk.risk_score);
                        let last_price = item
                            .last_price
                            .map(|price| format!("{:.2}", price))
                            .unwrap_or_else(|| "-".to_string());
                        let pct_display = item
                            .price_change_pct
                            .map(|pct| format!("{:+.2}%", pct))
                            .unwrap_or_else(|| "-".to_string());
                        let pct_class = pct_change_class(item.price_change_pct);
                        view! {
                            <div class=format!("rounded-xl border border-slate-200 p-4 shadow-sm {}", card_bg)>
                                <div class="flex items-start justify-between">
                                    <div>
                                        <div class="text-xs uppercase tracking-wide text-slate-500">{category.clone()}</div>
                                        <div class="text-lg font-semibold text-slate-900">{item.symbol.clone()}</div>
                                    </div>
                                    <span class=format!("inline-flex items-center rounded-full px-2 py-0.5 text-xs font-semibold {}", risk_badge)>
                                        {format!("Risk {}", item.risk.risk_score)}
                                    </span>
                                </div>
                                <div class="mt-4 flex items-end justify-between">
                                    <div>
                                        <div class="text-xs text-slate-500">"Last price"</div>
                                        <div class="text-xl font-semibold text-slate-900">{last_price}</div>
                                    </div>
                                    <div class="text-right">
                                        <div class="text-xs text-slate-500">"% Chg"</div>
                                        <div class=format!("text-lg font-semibold {}", pct_class)>{pct_display}</div>
                                    </div>
                                </div>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
    .into_any()
}
