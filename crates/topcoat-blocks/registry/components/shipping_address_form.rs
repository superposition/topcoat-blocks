#![allow(clippy::too_many_arguments, clippy::too_many_lines)]

use super::{button::button, input::input, label::label};
use topcoat::{
    Result,
    view::{Attributes, View, attributes, class, component, view},
};

/// Existing values rendered into [`shipping_address_form`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ShippingAddress {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub line1: String,
    pub line2: String,
    pub city: String,
    pub region: String,
    pub postal_code: String,
    pub country_code: String,
    pub country_label: String,
}

/// One host-owned validation error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AddressFieldError {
    pub field: String,
    pub message: String,
}

impl AddressFieldError {
    #[must_use]
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }
}

fn field_error<'a>(errors: &'a [AddressFieldError], field: &str) -> Option<&'a str> {
    errors
        .iter()
        .find(|error| error.field == field)
        .map(|error| error.message.as_str())
}

/// Accessible server POST form for a physical shipping address.
///
/// The host validates, normalizes, persists, and decides which countries are
/// allowed. `child` is intended for CSRF and checkout identifiers.
#[component]
pub async fn shipping_address_form(
    #[into] action: String,
    address: ShippingAddress,
    #[default] errors: Vec<AddressFieldError>,
    #[default]
    #[into]
    id_prefix: String,
    #[default]
    #[into]
    submit_label: String,
    #[default] pending: bool,
    #[default] mut attrs: Attributes,
    #[default] child: View,
) -> Result {
    let prefix = if id_prefix.is_empty() {
        "shipping".to_owned()
    } else {
        id_prefix
    };
    let submit_label = if submit_label.is_empty() {
        "Continue to payment".to_owned()
    } else {
        submit_label
    };

    let name_id = format!("{prefix}-name");
    let email_id = format!("{prefix}-email");
    let phone_id = format!("{prefix}-phone");
    let line1_id = format!("{prefix}-line1");
    let line2_id = format!("{prefix}-line2");
    let city_id = format!("{prefix}-city");
    let region_id = format!("{prefix}-region");
    let postal_id = format!("{prefix}-postal-code");

    view! {
        <form
            method="post"
            action=(action)
            data-topcoat-block="shipping-address-form"
            class=(class!("grid gap-6", attrs.remove("class")))
            (attrs)
        >
            (child)
            <input type="hidden" name="country" value=(address.country_code.as_str())>
            match errors.is_empty() {
                true => {},
                false => <div
                    role="alert"
                    class="rounded-lg border border-destructive/40 bg-destructive/10 p-4 text-sm text-destructive"
                >
                    <p class="font-medium">"Check the highlighted address fields."</p>
                    <ul class="mt-2 list-disc space-y-1 pl-5">
                        for error in errors.iter() {
                            <li>(error.message.as_str())</li>
                        }
                    </ul>
                </div>,
            }
            <div class="grid gap-5 sm:grid-cols-2">
                <div class="grid gap-2 sm:col-span-2">
                    label(attrs: attributes! { for=(name_id.as_str()) }, "Full name")
                    input(attrs: attributes! {
                        id=(name_id)
                        name="name"
                        value=(address.name.as_str())
                        autocomplete="name"
                        required=""
                    })
                    match field_error(&errors, "name") {
                        Some(error) => <p class="text-sm text-destructive">(error)</p>,
                        None => {},
                    }
                </div>
                <div class="grid gap-2">
                    label(attrs: attributes! { for=(email_id.as_str()) }, "Email")
                    input(attrs: attributes! {
                        id=(email_id)
                        name="email"
                        type="email"
                        inputmode="email"
                        value=(address.email.as_str())
                        autocomplete="email"
                        required=""
                    })
                    match field_error(&errors, "email") {
                        Some(error) => <p class="text-sm text-destructive">(error)</p>,
                        None => {},
                    }
                </div>
                <div class="grid gap-2">
                    label(attrs: attributes! { for=(phone_id.as_str()) }, "Phone")
                    input(attrs: attributes! {
                        id=(phone_id)
                        name="phone"
                        type="tel"
                        inputmode="tel"
                        value=(address.phone.as_str())
                        autocomplete="tel"
                        required=""
                    })
                    match field_error(&errors, "phone") {
                        Some(error) => <p class="text-sm text-destructive">(error)</p>,
                        None => {},
                    }
                </div>
                <div class="grid gap-2 sm:col-span-2">
                    label(attrs: attributes! { for=(line1_id.as_str()) }, "Street address")
                    input(attrs: attributes! {
                        id=(line1_id)
                        name="line1"
                        value=(address.line1.as_str())
                        autocomplete="shipping address-line1"
                        required=""
                    })
                    match field_error(&errors, "line1") {
                        Some(error) => <p class="text-sm text-destructive">(error)</p>,
                        None => {},
                    }
                </div>
                <div class="grid gap-2 sm:col-span-2">
                    label(attrs: attributes! { for=(line2_id.as_str()) }, "Apartment, suite, or unit")
                    input(attrs: attributes! {
                        id=(line2_id)
                        name="line2"
                        value=(address.line2.as_str())
                        autocomplete="shipping address-line2"
                    })
                </div>
                <div class="grid gap-2 sm:col-span-2">
                    label(attrs: attributes! { for=(city_id.as_str()) }, "City")
                    input(attrs: attributes! {
                        id=(city_id)
                        name="city"
                        value=(address.city.as_str())
                        autocomplete="shipping address-level2"
                        required=""
                    })
                    match field_error(&errors, "city") {
                        Some(error) => <p class="text-sm text-destructive">(error)</p>,
                        None => {},
                    }
                </div>
                <div class="grid gap-2">
                    label(attrs: attributes! { for=(region_id.as_str()) }, "State or region")
                    input(attrs: attributes! {
                        id=(region_id)
                        name="region"
                        value=(address.region.as_str())
                        autocomplete="shipping address-level1"
                        required=""
                    })
                    match field_error(&errors, "region") {
                        Some(error) => <p class="text-sm text-destructive">(error)</p>,
                        None => {},
                    }
                </div>
                <div class="grid gap-2">
                    label(attrs: attributes! { for=(postal_id.as_str()) }, "Postal code")
                    input(attrs: attributes! {
                        id=(postal_id)
                        name="postal_code"
                        value=(address.postal_code.as_str())
                        autocomplete="shipping postal-code"
                        inputmode="numeric"
                        required=""
                    })
                    match field_error(&errors, "postal_code") {
                        Some(error) => <p class="text-sm text-destructive">(error)</p>,
                        None => {},
                    }
                </div>
            </div>
            <p class="text-sm text-muted-foreground">
                "Shipping country: "
                <strong class="font-medium text-foreground">(address.country_label.as_str())</strong>
            </p>
            match pending {
                true => button(
                    attrs: attributes! {
                        type="submit"
                        disabled=""
                        aria-busy="true"
                        class="min-h-11 w-full sm:w-auto"
                    },
                    "Validating address…"
                ),
                false => button(
                    attrs: attributes! {
                        type="submit"
                        class="min-h-11 w-full sm:w-auto"
                    },
                    (submit_label)
                ),
            }
        </form>
    }
}
