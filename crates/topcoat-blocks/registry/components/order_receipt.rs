use super::{
    badge::{BadgeVariant, badge},
    button::{ButtonSize, ButtonVariant, button_variants},
    card::{card, card_content, card_footer, card_header, card_title},
};
use topcoat::{
    Result,
    view::{Attributes, attributes, class, component, view},
};

/// One immutable order line shown in a receipt.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReceiptLine {
    pub title: String,
    pub quantity: u32,
    pub total: String,
}

/// Host-owned receipt data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrderReceipt {
    pub order_number: String,
    pub status: String,
    pub placed_at: String,
    pub lines: Vec<ReceiptLine>,
    pub total: String,
    pub shipping_address: Vec<String>,
    pub tracking_label: Option<String>,
    pub tracking_href: Option<String>,
    pub continue_href: String,
}

/// Read-only order confirmation and recovery surface.
#[component]
pub async fn order_receipt(
    receipt: OrderReceipt,
    #[default] mut attrs: Attributes,
) -> Result {
    view! {
        <section
            data-topcoat-block="order-receipt"
            class=(class!("w-full", attrs.remove("class")))
            (attrs)
        >
            card(
                card_header(
                    <div class="flex flex-wrap items-start justify-between gap-4">
                        <div>
                            <p class="text-sm text-muted-foreground">
                                "Order "
                                (receipt.order_number.as_str())
                            </p>
                            card_title(attrs: attributes! { class="mt-1" }, "Order confirmed")
                            <p class="mt-2 text-sm text-muted-foreground">
                                (receipt.placed_at.as_str())
                            </p>
                        </div>
                        badge(variant: BadgeVariant::Secondary, (receipt.status.as_str()))
                    </div>
                )
                card_content(
                    attrs: attributes! { class="grid gap-6" },
                    <div>
                        <h3 class="text-sm font-semibold">"Items"</h3>
                        <ul class="mt-3 divide-y divide-border border-y border-border">
                            for line in receipt.lines {
                                <li class="flex justify-between gap-4 py-3 text-sm">
                                    <span>
                                        (line.title)
                                        " × "
                                        (line.quantity)
                                    </span>
                                    <span class="font-medium">(line.total)</span>
                                </li>
                            }
                        </ul>
                        <p class="mt-4 flex justify-between gap-4 font-semibold">
                            <span>"Total"</span>
                            <span>(receipt.total.as_str())</span>
                        </p>
                    </div>
                    <div>
                        <h3 class="text-sm font-semibold">"Ship to"</h3>
                        <address class="mt-2 text-sm leading-6 text-muted-foreground not-italic">
                            for line in receipt.shipping_address {
                                <span class="block">(line)</span>
                            }
                        </address>
                    </div>
                )
                card_footer(
                    attrs: attributes! { class="flex flex-wrap gap-3 border-t border-border pt-5" },
                    match (receipt.tracking_href.as_deref(), receipt.tracking_label.as_deref()) {
                        (Some(href), Some(label)) => <a
                            href=(href)
                            rel="external noreferrer"
                            class=(button_variants(ButtonVariant::Outline, ButtonSize::Md))
                        >
                            (label)
                        </a>,
                        _ => {},
                    }
                    <a
                        href=(receipt.continue_href.as_str())
                        class=(button_variants(ButtonVariant::Primary, ButtonSize::Md))
                    >
                        "Continue shopping"
                    </a>
                )
            )
        </section>
    }
}
