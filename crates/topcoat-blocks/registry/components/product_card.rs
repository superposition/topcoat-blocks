#![allow(clippy::too_many_lines)]

use super::{
    badge::{BadgeVariant, badge},
    button::button,
    card::{card, card_content, card_footer, card_header, card_title},
};
use topcoat::{
    Result,
    view::{Attributes, View, attributes, class, component, view},
};

/// Inventory state presented by [`product_card`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ProductAvailability {
    /// The item can be purchased normally.
    #[default]
    Available,
    /// The item can be purchased, but inventory is constrained.
    LowStock,
    /// The item cannot currently be purchased.
    SoldOut,
    /// Product availability is still being resolved.
    Loading,
}

impl ProductAvailability {
    /// Stable value rendered to `data-state`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::LowStock => "low-stock",
            Self::SoldOut => "sold-out",
            Self::Loading => "loading",
        }
    }

    /// Human-readable status label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Available => "Available",
            Self::LowStock => "Low stock",
            Self::SoldOut => "Sold out",
            Self::Loading => "Checking stock",
        }
    }

    /// Whether the add-to-cart action may be submitted.
    #[must_use]
    pub const fn can_purchase(self) -> bool {
        matches!(self, Self::Available | Self::LowStock)
    }

    const fn badge_variant(self) -> BadgeVariant {
        match self {
            Self::Available => BadgeVariant::Secondary,
            Self::LowStock | Self::Loading => BadgeVariant::Outline,
            Self::SoldOut => BadgeVariant::Destructive,
        }
    }
}

/// Host-owned product content rendered by [`product_card`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProductCard {
    pub href: String,
    pub image_src: Option<String>,
    pub image_alt: String,
    pub eyebrow: Option<String>,
    pub title: String,
    pub summary: String,
    pub price: String,
    pub compare_at_price: Option<String>,
    pub availability: ProductAvailability,
}

/// Portable catalog card with an optional native add-to-cart form.
///
/// The host owns product retrieval, pricing, inventory, the POST action, CSRF,
/// and every hidden identifier. Place those hidden fields in `child`.
#[component]
pub async fn product_card(
    product: ProductCard,
    #[default] add_action: Option<String>,
    #[default] mut attrs: Attributes,
    #[default] child: View,
) -> Result {
    let state = product.availability.as_str();
    let can_purchase = product.availability.can_purchase();

    view! {
        <article
            data-topcoat-block="product-card"
            data-state=(state)
            class=(class!("h-full", attrs.remove("class")))
            (attrs)
        >
            card(
                attrs: attributes! { class="flex h-full flex-col overflow-hidden" },
                <a
                    href=(product.href.as_str())
                    aria-label=(format!("View {}", product.title))
                    class="group block aspect-[4/3] overflow-hidden bg-muted focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:outline-none"
                >
                    match product.image_src.as_deref() {
                        Some(src) => <img
                            src=(src)
                            alt=(product.image_alt.as_str())
                            loading="lazy"
                            class="h-full w-full object-cover transition-transform duration-200 group-hover:scale-[1.02]"
                        >,
                        None => <span
                            aria-hidden="true"
                            class="flex h-full items-center justify-center text-xs font-medium tracking-[0.16em] text-muted-foreground uppercase"
                        >
                            "No image"
                        </span>,
                    }
                </a>
                card_header(
                    attrs: attributes! { class="gap-3" },
                    <div class="flex items-start justify-between gap-3">
                        <div class="min-w-0">
                            match product.eyebrow.as_deref() {
                                Some(eyebrow) => <p class="mb-2 text-xs font-medium tracking-[0.14em] text-muted-foreground uppercase">
                                    (eyebrow)
                                </p>,
                                None => {},
                            }
                            card_title(
                                <a
                                    href=(product.href.as_str())
                                    class="rounded-sm underline-offset-4 hover:underline focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none"
                                >
                                    (product.title.as_str())
                                </a>
                            )
                        </div>
                        badge(
                            variant: product.availability.badge_variant(),
                            attrs: attributes! { data-availability=(state) },
                            (product.availability.label())
                        )
                    </div>
                )
                card_content(
                    attrs: attributes! { class="flex-1" },
                    <p class="text-sm leading-6 text-muted-foreground">
                        (product.summary.as_str())
                    </p>
                )
                card_footer(
                    attrs: attributes! { class="flex items-end justify-between gap-4 border-t border-border pt-5" },
                    <p class="grid gap-0.5">
                        <span class="text-base font-semibold">(product.price.as_str())</span>
                        match product.compare_at_price.as_deref() {
                            Some(price) => <del class="text-xs text-muted-foreground">(price)</del>,
                            None => {},
                        }
                    </p>
                    match add_action {
                        Some(action) => <form method="post" action=(action) class="shrink-0">
                            (child)
                            match can_purchase {
                                true => button(
                                    attrs: attributes! {
                                        type="submit"
                                        name="intent"
                                        value="add"
                                        class="min-h-11"
                                    },
                                    "Add to cart"
                                ),
                                false => button(
                                    attrs: attributes! {
                                        type="button"
                                        disabled=""
                                        class="min-h-11"
                                    },
                                    "Unavailable"
                                ),
                            }
                        </form>,
                        None => {},
                    }
                )
            )
        </article>
    }
}
