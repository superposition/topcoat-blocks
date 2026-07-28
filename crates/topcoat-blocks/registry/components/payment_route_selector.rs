use super::{button::button, card::card};
use topcoat::{
    Result,
    view::{Attributes, View, attributes, class, component, view},
};

/// Host-owned payment route rendered by [`payment_route_selector`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PaymentRouteOption {
    pub id: String,
    pub network: String,
    pub asset: String,
    pub amount: String,
    pub fee: String,
    pub timing: String,
    pub available: bool,
    pub selected: bool,
    pub unavailable_reason: Option<String>,
}

/// Native single-choice payment route form.
///
/// The host owns wallets, risk checks, route eligibility, intent creation, and
/// every hidden value in `child`.
#[component]
pub async fn payment_route_selector(
    #[into] action: String,
    routes: Vec<PaymentRouteOption>,
    #[default] mut attrs: Attributes,
    #[default] child: View,
) -> Result {
    let has_available_route = routes.iter().any(|route| route.available);

    view! {
        <form
            method="post"
            action=(action)
            data-topcoat-block="payment-route-selector"
            class=(class!("grid gap-5", attrs.remove("class")))
            (attrs)
        >
            (child)
            <fieldset class="grid gap-3">
                <legend class="text-lg font-semibold">"Choose a payment network"</legend>
                <p class="text-sm leading-6 text-muted-foreground">
                    "The host verifies the final amount and network before creating an intent."
                </p>
                for route in routes {
                    card(
                        attrs: attributes! {
                            class="relative p-0"
                            data-route-id=(route.id.as_str())
                            data-state=(if route.available { "available" } else { "unavailable" })
                        },
                        <label class="flex cursor-pointer items-start gap-4 p-5 has-[:disabled]:cursor-not-allowed has-[:disabled]:opacity-60">
                            match (route.available, route.selected) {
                                (true, true) => <input
                                    type="radio"
                                    name="route_id"
                                    value=(route.id.as_str())
                                    checked=""
                                    required=""
                                    class="mt-1 size-4"
                                >,
                                (true, false) => <input
                                    type="radio"
                                    name="route_id"
                                    value=(route.id.as_str())
                                    required=""
                                    class="mt-1 size-4"
                                >,
                                (false, _) => <input
                                    type="radio"
                                    name="route_id"
                                    value=(route.id.as_str())
                                    disabled=""
                                    class="mt-1 size-4"
                                >,
                            }
                            <span class="grid min-w-0 flex-1 gap-2">
                                <span class="flex flex-wrap items-baseline justify-between gap-2">
                                    <strong>(route.network.as_str())</strong>
                                    <span class="font-semibold">(route.amount.as_str())</span>
                                </span>
                                <span class="text-sm text-muted-foreground">
                                    (route.asset.as_str())
                                    " · "
                                    (route.fee.as_str())
                                    " · "
                                    (route.timing.as_str())
                                </span>
                                match route.unavailable_reason.as_deref() {
                                    Some(reason) => <span class="text-sm text-destructive">(reason)</span>,
                                    None => {},
                                }
                            </span>
                        </label>
                    )
                }
            </fieldset>
            match has_available_route {
                true => button(
                    attrs: attributes! {
                        type="submit"
                        class="min-h-11 w-full sm:w-auto"
                    },
                    "Continue to wallet"
                ),
                false => button(
                    attrs: attributes! {
                        type="button"
                        disabled=""
                        class="min-h-11 w-full sm:w-auto"
                    },
                    "No route available"
                ),
            }
        </form>
    }
}
