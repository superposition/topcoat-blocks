pub mod components;

use coatcheck::{Catalog, Story, StoryKind};
use components::{
    cart_line_item::{CartLineItem, cart_line_item},
    catalog_filter_bar::{CatalogFilterOption, catalog_filter_bar},
    order_receipt::{OrderReceipt, ReceiptLine, order_receipt},
    order_totals::{TotalLine, TotalLineTone, order_totals},
    payment_route_selector::{PaymentRouteOption, payment_route_selector},
    payment_status::{PaymentState, PaymentStatus, payment_status},
    product_card::{ProductAvailability, ProductCard, product_card},
    shipping_address_form::{AddressFieldError, ShippingAddress, shipping_address_form},
    sign_in_form::sign_in_form,
};
use topcoat::{
    Result,
    asset::{AssetBundle, RouterBuilderAssetExt},
    context::Cx,
    router::{Json, Router, Slot, layout, page, query_params, route},
    tailwind,
    view::view,
};

const STORIES: &[Story] = &[
    Story {
        id: "sign-in-default",
        title: "Sign-in form",
        tier: "Authentication",
        kind: StoryKind::Playground,
        description: "Portable POST form with recovery and account-creation paths.",
        source: "crates/topcoat-blocks/registry/components/sign_in_form.rs",
        args: &[],
    },
    Story {
        id: "sign-in-minimal",
        title: "Sign-in form / minimal",
        tier: "Authentication",
        kind: StoryKind::State,
        description: "The same form contract without optional navigation.",
        source: "crates/topcoat-blocks/registry/components/sign_in_form.rs",
        args: &[],
    },
    Story {
        id: "product-card-available",
        title: "Product card",
        tier: "Commerce",
        kind: StoryKind::Playground,
        description: "Available product with a native add-to-cart POST.",
        source: "crates/topcoat-blocks/registry/components/product_card.rs",
        args: &[],
    },
    Story {
        id: "product-card-sold-out",
        title: "Product card / sold out",
        tier: "Commerce",
        kind: StoryKind::State,
        description: "Unavailable inventory keeps the action disabled.",
        source: "crates/topcoat-blocks/registry/components/product_card.rs",
        args: &[],
    },
    Story {
        id: "catalog-filter-bar",
        title: "Catalog filter bar",
        tier: "Commerce",
        kind: StoryKind::Playground,
        description: "Native GET search, category, and sorting controls.",
        source: "crates/topcoat-blocks/registry/components/catalog_filter_bar.rs",
        args: &[],
    },
    Story {
        id: "cart-line-item",
        title: "Cart line item",
        tier: "Commerce",
        kind: StoryKind::Playground,
        description: "Quantity update and removal through one POST contract.",
        source: "crates/topcoat-blocks/registry/components/cart_line_item.rs",
        args: &[],
    },
    Story {
        id: "order-totals",
        title: "Order totals",
        tier: "Commerce",
        kind: StoryKind::Playground,
        description: "Host-formatted subtotal, shipping, tax, and total.",
        source: "crates/topcoat-blocks/registry/components/order_totals.rs",
        args: &[],
    },
    Story {
        id: "shipping-address",
        title: "Shipping address form",
        tier: "Commerce",
        kind: StoryKind::Playground,
        description: "Server POST form for a physical shipping address.",
        source: "crates/topcoat-blocks/registry/components/shipping_address_form.rs",
        args: &[],
    },
    Story {
        id: "shipping-address-errors",
        title: "Shipping address / errors",
        tier: "Commerce",
        kind: StoryKind::State,
        description: "Host validation errors remain visible and semantic.",
        source: "crates/topcoat-blocks/registry/components/shipping_address_form.rs",
        args: &[],
    },
    Story {
        id: "payment-routes",
        title: "Payment route selector",
        tier: "Commerce",
        kind: StoryKind::Playground,
        description: "Native single-choice network selection.",
        source: "crates/topcoat-blocks/registry/components/payment_route_selector.rs",
        args: &[],
    },
    Story {
        id: "payment-confirming",
        title: "Payment status / confirming",
        tier: "Commerce",
        kind: StoryKind::State,
        description: "Live region for a transfer awaiting confirmations.",
        source: "crates/topcoat-blocks/registry/components/payment_status.rs",
        args: &[],
    },
    Story {
        id: "payment-paid",
        title: "Payment status / paid",
        tier: "Commerce",
        kind: StoryKind::State,
        description: "Confirmed transfer with explorer and recovery links.",
        source: "crates/topcoat-blocks/registry/components/payment_status.rs",
        args: &[],
    },
    Story {
        id: "order-receipt",
        title: "Order receipt",
        tier: "Commerce",
        kind: StoryKind::Playground,
        description: "Read-only confirmation, shipment, and recovery surface.",
        source: "crates/topcoat-blocks/registry/components/order_receipt.rs",
        args: &[],
    },
];
const CATALOG: Catalog = Catalog::new("Topcoat Blocks", STORIES);

#[tokio::main]
async fn main() {
    CATALOG
        .validate()
        .expect("the Topcoat Blocks story catalog is valid");

    let router = Router::builder()
        .layout(root_layout)
        .page(home)
        .page(frame)
        .route(health)
        .route(coatcheck_manifest)
        .assets(AssetBundle::load().expect("workbench assets load"))
        .build();

    topcoat::start(router)
        .await
        .expect("the Topcoat Blocks workbench starts");
}

#[layout("/")]
async fn root_layout(slot: Slot<'_>) -> Result {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <meta
                    name="description"
                    content="Visual development and semantic review for Topcoat Blocks."
                >
                <title>"Topcoat Blocks / Coatcheck"</title>
                topcoat::dev::script()
                <link rel="stylesheet" href=(tailwind::stylesheet!())>
            </head>
            <body>(slot.await?)</body>
        </html>
    }
}

#[page("/")]
#[allow(clippy::too_many_lines)]
async fn home() -> Result {
    let registry_name = topcoat_blocks::REGISTRY_NAME;

    view! {
        <main class="min-h-svh bg-background px-5 py-8 text-foreground sm:px-8 lg:px-12">
            <div class="mx-auto max-w-6xl">
                <header class="border-b border-border pb-8">
                    <p class="text-xs font-semibold tracking-[0.18em] text-muted-foreground uppercase">
                        "Coatcheck review host"
                    </p>
                    <div class="mt-3 flex flex-col justify-between gap-5 md:flex-row md:items-end">
                        <div class="max-w-3xl">
                            <h1 class="text-4xl font-semibold tracking-tight sm:text-5xl">
                                "Topcoat Blocks"
                            </h1>
                            <p class="mt-4 max-w-2xl text-base leading-7 text-muted-foreground">
                                "Registry source enters here, survives responsive and semantic \
                                 review, then ships as copy-to-own Rust."
                            </p>
                        </div>
                        <code class="w-fit rounded-md border border-border bg-muted px-3 py-2 text-sm">
                            (registry_name)
                        </code>
                    </div>
                </header>

                <section class="grid gap-4 py-8 md:grid-cols-3" aria-label="Review contract">
                    <article class="rounded-xl border border-border bg-background p-5 shadow-sm">
                        <p class="text-sm font-medium">"1 / Build"</p>
                        <p class="mt-2 text-sm leading-6 text-muted-foreground">
                            "Develop one canonical registry source file."
                        </p>
                    </article>
                    <article class="rounded-xl border border-border bg-background p-5 shadow-sm">
                        <p class="text-sm font-medium">"2 / Inspect"</p>
                        <p class="mt-2 text-sm leading-6 text-muted-foreground">
                            "Review real installs across viewports and surfaces."
                        </p>
                    </article>
                    <article class="rounded-xl border border-border bg-background p-5 shadow-sm">
                        <p class="text-sm font-medium">"3 / Release"</p>
                        <p class="mt-2 text-sm leading-6 text-muted-foreground">
                            "Publish only after semantic and visual approval."
                        </p>
                    </article>
                </section>

                <section class="rounded-2xl border border-border bg-muted/30 p-4 sm:p-6">
                    <div class="flex flex-col gap-4">
                        <div>
                            <h2 class="text-lg font-semibold">"Authentication"</h2>
                            <p class="mt-1 text-sm text-muted-foreground">
                                "The workbench renders the installed registry source, never a \
                                 parallel demo implementation."
                            </p>
                        </div>
                        <div class="grid gap-3 md:grid-cols-2">
                            <a
                                href="/__coatcheck/frame?story=sign-in-default&scene=desktop-light"
                                class="group rounded-xl border border-border bg-background p-4 shadow-xs transition-colors hover:bg-muted focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background focus-visible:outline-none"
                            >
                                <span class="block text-sm font-medium">"Sign-in form"</span>
                                <span class="mt-1 block text-sm leading-6 text-muted-foreground">
                                    "Recovery and account-creation paths."
                                </span>
                            </a>
                            <a
                                href="/__coatcheck/frame?story=sign-in-minimal&scene=desktop-light"
                                class="group rounded-xl border border-border bg-background p-4 shadow-xs transition-colors hover:bg-muted focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background focus-visible:outline-none"
                            >
                                <span class="block text-sm font-medium">"Sign-in form / minimal"</span>
                                <span class="mt-1 block text-sm leading-6 text-muted-foreground">
                                    "Only the required authentication controls."
                                </span>
                            </a>
                        </div>
                    </div>
                </section>

                <section class="mt-6 rounded-2xl border border-border bg-muted/30 p-4 sm:p-6">
                    <div class="flex flex-col gap-4">
                        <div>
                            <h2 class="text-lg font-semibold">"Commerce"</h2>
                            <p class="mt-1 text-sm text-muted-foreground">
                                "Provider-neutral product, cart, checkout, payment, and receipt contracts."
                            </p>
                        </div>
                        <div class="grid gap-3 md:grid-cols-2 lg:grid-cols-3">
                            for (story, title) in [
                                ("product-card-available", "Product card"),
                                ("catalog-filter-bar", "Catalog filter bar"),
                                ("cart-line-item", "Cart line item"),
                                ("order-totals", "Order totals"),
                                ("shipping-address", "Shipping address"),
                                ("payment-routes", "Payment routes"),
                                ("payment-confirming", "Payment status"),
                                ("order-receipt", "Order receipt"),
                            ] {
                                <a
                                    href=(format!("/__coatcheck/frame?story={story}&scene=desktop-light"))
                                    class="rounded-xl border border-border bg-background p-4 shadow-xs transition-colors hover:bg-muted focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:outline-none"
                                >
                                    <span class="block text-sm font-medium">(title)</span>
                                    <span class="mt-1 block text-sm text-muted-foreground">
                                        "Open semantic review frame"
                                    </span>
                                </a>
                            }
                        </div>
                    </div>
                </section>
            </div>
        </main>
    }
}

#[query_params(error = bad_request)]
struct FrameQuery {
    story: String,
    #[serde(default)]
    scene: Option<String>,
}

#[page("/__coatcheck/frame")]
#[allow(clippy::too_many_lines)]
async fn frame(cx: &Cx) -> Result {
    let query = query_params::<FrameQuery>(cx)?;
    let scene = query.scene.as_deref().unwrap_or("desktop-light");
    let dark = scene.ends_with("-dark");

    if CATALOG.story(&query.story).is_none() {
        return view! {
            <main
                data-coatcheck-frame=""
                data-coatcheck-error=""
                role="alert"
                class="min-h-svh bg-background p-8 text-foreground"
            >
                <h1 class="text-xl font-semibold">"Unknown story"</h1>
                <p class="mt-2 text-muted-foreground">(query.story.as_str())</p>
            </main>
        };
    }

    let class_name = if dark {
        "dark min-h-svh bg-background text-foreground"
    } else {
        "min-h-svh bg-background text-foreground"
    };

    view! {
        <main
            data-coatcheck-frame=""
            data-story-id=(query.story.as_str())
            data-scene-id=(scene)
            class=(class_name)
        >
            <div class="min-h-svh bg-muted/40 p-4 sm:p-8">
                <h1 class="sr-only">"Topcoat block review"</h1>
                <div class="mx-auto flex min-h-[calc(100svh-4rem)] max-w-5xl items-center justify-center">
                match query.story.as_str() {
                    "sign-in-default" => sign_in_form(
                        action: "/session",
                        forgot_password_href: Some("/forgot-password".to_owned()),
                        sign_up_href: Some("/sign-up".to_owned()),
                        attrs: topcoat::view::attributes! {
                            class="max-w-sm"
                            data-review-subject="sign-in-form"
                        },
                        <input type="hidden" name="csrf_token" value="review-token">
                    ),
                    "sign-in-minimal" => sign_in_form(
                        action: "/session",
                        id_prefix: "minimal",
                        attrs: topcoat::view::attributes! {
                            class="max-w-sm"
                            data-review-subject="sign-in-form"
                        }
                    ),
                    "product-card-available" => product_card(
                        product: sample_product(ProductAvailability::Available),
                        add_action: Some("/cart/items".to_owned()),
                        attrs: topcoat::view::attributes! {
                            class="max-w-sm"
                            data-review-subject="product-card"
                        },
                        <input type="hidden" name="product_id" value="review-product">
                        <input type="hidden" name="csrf_token" value="review-token">
                    ),
                    "product-card-sold-out" => product_card(
                        product: sample_product(ProductAvailability::SoldOut),
                        add_action: Some("/cart/items".to_owned()),
                        attrs: topcoat::view::attributes! {
                            class="max-w-sm"
                            data-review-subject="product-card"
                        }
                    ),
                    "catalog-filter-bar" => catalog_filter_bar(
                        action: "/shop",
                        query: "field",
                        categories: vec![
                            CatalogFilterOption::new("", "All categories", false),
                            CatalogFilterOption::new("packs", "Packs", true),
                            CatalogFilterOption::new("apparel", "Apparel", false),
                        ],
                        sorts: vec![
                            CatalogFilterOption::new("featured", "Featured", true),
                            CatalogFilterOption::new("price-asc", "Price: low to high", false),
                        ],
                        attrs: topcoat::view::attributes! {
                            class="w-full"
                            data-review-subject="catalog-filter-bar"
                        }
                    ),
                    "cart-line-item" => cart_line_item(
                        item: sample_cart_line(),
                        action: "/cart/items/review-line",
                        id_prefix: "review-line",
                        attrs: topcoat::view::attributes! {
                            class="w-full"
                            data-review-subject="cart-line-item"
                        },
                        <input type="hidden" name="line_id" value="review-line">
                        <input type="hidden" name="csrf_token" value="review-token">
                    ),
                    "order-totals" => order_totals(
                        lines: sample_totals(),
                        action: Some("/checkout".to_owned()),
                        action_label: "Continue to checkout",
                        attrs: topcoat::view::attributes! {
                            class="max-w-md"
                            data-review-subject="order-totals"
                        },
                        <p class="text-xs leading-5 text-muted-foreground">
                            "Final shipping and tax are owned by the checkout host."
                        </p>
                    ),
                    "shipping-address" => shipping_address_form(
                        action: "/checkout/address",
                        address: sample_address(),
                        attrs: topcoat::view::attributes! {
                            class="w-full max-w-2xl rounded-xl border border-border bg-background p-5 sm:p-7"
                            data-review-subject="shipping-address-form"
                        },
                        <input type="hidden" name="csrf_token" value="review-token">
                    ),
                    "shipping-address-errors" => shipping_address_form(
                        action: "/checkout/address",
                        address: sample_address(),
                        errors: vec![
                            AddressFieldError::new("line1", "Enter a deliverable street address."),
                            AddressFieldError::new("postal_code", "Enter a valid postal code."),
                        ],
                        id_prefix: "shipping-errors",
                        attrs: topcoat::view::attributes! {
                            class="w-full max-w-2xl rounded-xl border border-border bg-background p-5 sm:p-7"
                            data-review-subject="shipping-address-form"
                        }
                    ),
                    "payment-routes" => payment_route_selector(
                        action: "/payments/intents",
                        routes: sample_payment_routes(),
                        attrs: topcoat::view::attributes! {
                            class="w-full max-w-2xl rounded-xl border border-border bg-background p-5 sm:p-7"
                            data-review-subject="payment-route-selector"
                        },
                        <input type="hidden" name="csrf_token" value="review-token">
                    ),
                    "payment-confirming" => payment_status(
                        status: sample_payment_status(PaymentState::Confirming),
                        attrs: topcoat::view::attributes! {
                            class="max-w-xl"
                            data-review-subject="payment-status"
                        }
                    ),
                    "payment-paid" => payment_status(
                        status: sample_payment_status(PaymentState::Paid),
                        attrs: topcoat::view::attributes! {
                            class="max-w-xl"
                            data-review-subject="payment-status"
                        }
                    ),
                    "order-receipt" => order_receipt(
                        receipt: sample_receipt(),
                        attrs: topcoat::view::attributes! {
                            class="max-w-2xl"
                            data-review-subject="order-receipt"
                        }
                    ),
                    _ => {},
                }
                </div>
            </div>
            <span data-coatcheck-ready="" hidden="">"ready"</span>
        </main>
    }
}

fn sample_product(availability: ProductAvailability) -> ProductCard {
    ProductCard {
        href: "/products/review-field-pack".to_owned(),
        image_src: None,
        image_alt: "Review product placeholder".to_owned(),
        eyebrow: Some("Field equipment".to_owned()),
        title: "Review field pack".to_owned(),
        summary: "A neutral product description supplied entirely by the host.".to_owned(),
        price: "$148.00".to_owned(),
        compare_at_price: None,
        availability,
    }
}

fn sample_cart_line() -> CartLineItem {
    CartLineItem {
        image_src: None,
        image_alt: "Review product placeholder".to_owned(),
        title: "Review field pack".to_owned(),
        variant: Some("Graphite / 24 L".to_owned()),
        unit_price: "$148.00".to_owned(),
        line_total: "$296.00".to_owned(),
        quantity: 2,
        max_quantity: 4,
    }
}

fn sample_totals() -> Vec<TotalLine> {
    vec![
        TotalLine::new("Subtotal", "$296.00", TotalLineTone::Standard),
        TotalLine::new("Shipping", "$12.40", TotalLineTone::Muted),
        TotalLine::new("Estimated tax", "$24.67", TotalLineTone::Muted),
        TotalLine::new("Total", "$333.07", TotalLineTone::Emphasis),
    ]
}

fn sample_address() -> ShippingAddress {
    ShippingAddress {
        name: "Morgan Field".to_owned(),
        email: "morgan@example.com".to_owned(),
        phone: "2125550142".to_owned(),
        line1: "120 Lafayette Street".to_owned(),
        line2: String::new(),
        city: "New York".to_owned(),
        region: "NY".to_owned(),
        postal_code: "10013".to_owned(),
        country_code: "US".to_owned(),
        country_label: "United States".to_owned(),
    }
}

fn sample_payment_routes() -> Vec<PaymentRouteOption> {
    vec![
        PaymentRouteOption {
            id: "solana-usdc".to_owned(),
            network: "Solana".to_owned(),
            asset: "USDC".to_owned(),
            amount: "333.07 USDC".to_owned(),
            fee: "Network fee paid by wallet".to_owned(),
            timing: "Usually under a minute".to_owned(),
            available: true,
            selected: true,
            unavailable_reason: None,
        },
        PaymentRouteOption {
            id: "ethereum-usdc".to_owned(),
            network: "Ethereum".to_owned(),
            asset: "USDC".to_owned(),
            amount: "333.07 USDC".to_owned(),
            fee: "Network fee paid by wallet".to_owned(),
            timing: "Confirmation time varies".to_owned(),
            available: true,
            selected: false,
            unavailable_reason: None,
        },
        PaymentRouteOption {
            id: "unavailable-route".to_owned(),
            network: "Unavailable network".to_owned(),
            asset: "USDC".to_owned(),
            amount: "333.07 USDC".to_owned(),
            fee: "Not quoted".to_owned(),
            timing: "Unavailable".to_owned(),
            available: false,
            selected: false,
            unavailable_reason: Some("The host has disabled this route.".to_owned()),
        },
    ]
}

fn sample_payment_status(state: PaymentState) -> PaymentStatus {
    let (title, message) = match state {
        PaymentState::Paid => (
            "Transfer confirmed",
            "The host verified the network, asset, recipient, amount, and finality.",
        ),
        _ => (
            "Confirming transfer",
            "The transfer was found and is waiting for the host confirmation policy.",
        ),
    };

    PaymentStatus {
        state,
        title: title.to_owned(),
        message: message.to_owned(),
        amount: "333.07 USDC".to_owned(),
        network: "Solana".to_owned(),
        recipient: Some("ReviewRecipient1111111111111111111111111111".to_owned()),
        transaction_href: Some("https://example.com/transaction/review".to_owned()),
        transaction_label: Some("View transaction".to_owned()),
        recovery_href: Some("/payments/recover/review".to_owned()),
    }
}

fn sample_receipt() -> OrderReceipt {
    OrderReceipt {
        order_number: "CF-000184".to_owned(),
        status: "Preparing shipment".to_owned(),
        placed_at: "July 27, 2026 at 14:35 UTC".to_owned(),
        lines: vec![
            ReceiptLine {
                title: "Review field pack".to_owned(),
                quantity: 2,
                total: "$296.00".to_owned(),
            },
            ReceiptLine {
                title: "Review utility strap".to_owned(),
                quantity: 1,
                total: "$24.00".to_owned(),
            },
        ],
        total: "$356.14".to_owned(),
        shipping_address: vec![
            "Morgan Field".to_owned(),
            "120 Lafayette Street".to_owned(),
            "New York, NY 10013".to_owned(),
            "United States".to_owned(),
        ],
        tracking_label: Some("Track shipment".to_owned()),
        tracking_href: Some("https://example.com/tracking/review".to_owned()),
        continue_href: "/shop".to_owned(),
    }
}

fn build_manifest() -> coatcheck::contract::ManifestV1 {
    use coatcheck::contract::{FrameContractV1, ManifestV1};

    let mut manifest = ManifestV1::from_catalog(CATALOG, FrameContractV1::default());
    manifest.scenes = vec![
        scene("mobile-light", "Mobile light", 393, 852, "light"),
        scene("tablet-light", "Tablet light", 768, 1024, "light"),
        scene("desktop-light", "Desktop light", 1280, 900, "light"),
        scene("mobile-dark", "Mobile dark", 393, 852, "dark"),
        scene("desktop-dark", "Desktop dark", 1280, 900, "dark"),
    ];
    manifest
}

fn scene(
    id: &str,
    label: &str,
    width: u16,
    height: u16,
    surface: &str,
) -> coatcheck::contract::SceneV1 {
    coatcheck::contract::SceneV1 {
        id: id.to_owned(),
        label: label.to_owned(),
        viewport: coatcheck::contract::ViewportV1 { width, height },
        surface: surface.to_owned(),
    }
}

#[route(GET "/__coatcheck/manifest.json")]
async fn coatcheck_manifest() -> Result<Json<coatcheck::contract::ManifestV1>> {
    let manifest = build_manifest();
    manifest
        .validate()
        .expect("the served Coatcheck manifest is valid");
    Ok(Json(manifest))
}

#[route(GET "/health")]
async fn health() -> Result<&'static str> {
    Ok("topcoat-blocks:ok")
}

#[cfg(test)]
mod tests {
    use super::{CATALOG, STORIES, build_manifest};

    #[test]
    fn catalog_and_manifest_are_valid() {
        assert_eq!(CATALOG.validate(), Ok(()));
        assert_eq!(build_manifest().validate(), Ok(()));
        assert_eq!(build_manifest().cases().len(), STORIES.len() * 5);
    }
}
