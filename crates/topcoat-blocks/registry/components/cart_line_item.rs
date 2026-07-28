use super::{button::button, input::input, label::label};
use topcoat::{
    Result,
    view::{Attributes, View, attributes, class, component, view},
};

/// Host-owned line content rendered by [`cart_line_item`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CartLineItem {
    pub image_src: Option<String>,
    pub image_alt: String,
    pub title: String,
    pub variant: Option<String>,
    pub unit_price: String,
    pub line_total: String,
    pub quantity: u32,
    pub max_quantity: u32,
}

/// Quantity and removal form for one cart line.
///
/// Both buttons submit to `action`. The host distinguishes them through
/// `intent=update` and `intent=remove`. Hidden line identifiers and CSRF input
/// belong in `child`.
#[component]
pub async fn cart_line_item(
    item: CartLineItem,
    #[into] action: String,
    #[default]
    #[into]
    id_prefix: String,
    #[default] mut attrs: Attributes,
    #[default] child: View,
) -> Result {
    let id_prefix = if id_prefix.is_empty() {
        "cart-line".to_owned()
    } else {
        id_prefix
    };
    let quantity_id = format!("{id_prefix}-quantity");
    let max_quantity = item.max_quantity.max(1);

    view! {
        <form
            method="post"
            action=(action)
            data-topcoat-block="cart-line-item"
            class=(class!("grid gap-4 border-b border-border py-5 sm:grid-cols-[6rem_minmax(0,1fr)_auto] sm:items-start", attrs.remove("class")))
            (attrs)
        >
            (child)
            <div class="aspect-square overflow-hidden rounded-lg bg-muted">
                match item.image_src.as_deref() {
                    Some(src) => <img
                        src=(src)
                        alt=(item.image_alt.as_str())
                        class="h-full w-full object-cover"
                    >,
                    None => <span
                        aria-hidden="true"
                        class="flex h-full items-center justify-center text-xs text-muted-foreground"
                    >
                        "No image"
                    </span>,
                }
            </div>
            <div class="min-w-0">
                <h3 class="font-medium">(item.title.as_str())</h3>
                match item.variant.as_deref() {
                    Some(variant) => <p class="mt-1 text-sm text-muted-foreground">(variant)</p>,
                    None => {},
                }
                <p class="mt-2 text-sm text-muted-foreground">
                    (item.unit_price.as_str())
                    " each"
                </p>
                <div class="mt-4 flex flex-wrap items-end gap-3">
                    <div class="grid gap-2">
                        label(
                            attrs: attributes! {
                                for=(quantity_id.as_str())
                                class="text-xs"
                            },
                            "Quantity"
                        )
                        input(attrs: attributes! {
                            id=(quantity_id)
                            name="quantity"
                            type="number"
                            min="1"
                            max=(max_quantity)
                            value=(item.quantity)
                            inputmode="numeric"
                            class="w-24"
                        })
                    </div>
                    button(
                        variant: super::button::ButtonVariant::Outline,
                        attrs: attributes! {
                            type="submit"
                            name="intent"
                            value="update"
                            class="min-h-10"
                        },
                        "Update"
                    )
                    button(
                        variant: super::button::ButtonVariant::Ghost,
                        attrs: attributes! {
                            type="submit"
                            name="intent"
                            value="remove"
                            class="min-h-10"
                        },
                        "Remove"
                    )
                </div>
            </div>
            <p class="font-semibold sm:text-right">(item.line_total.as_str())</p>
        </form>
    }
}
