use super::{
    button::{ButtonSize, ButtonVariant, button_variants},
    card::{card, card_content, card_footer, card_header, card_title},
};
use topcoat::{
    Result,
    view::{Attributes, View, attributes, class, component, view},
};

/// Visual weight for one monetary total.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TotalLineTone {
    #[default]
    Muted,
    Standard,
    Emphasis,
}

impl TotalLineTone {
    const fn classes(self) -> &'static str {
        match self {
            Self::Muted => "text-muted-foreground",
            Self::Standard => "text-foreground",
            Self::Emphasis => "border-t border-border pt-4 text-lg font-semibold text-foreground",
        }
    }
}

/// One host-formatted label and value in [`order_totals`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TotalLine {
    pub label: String,
    pub value: String,
    pub tone: TotalLineTone,
}

impl TotalLine {
    #[must_use]
    pub fn new(
        label: impl Into<String>,
        value: impl Into<String>,
        tone: TotalLineTone,
    ) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            tone,
        }
    }
}

/// Portable order-total summary with an optional navigation action.
///
/// Values are display strings. The host remains solely responsible for all
/// calculations, currency precision, quotes, and checkout eligibility.
#[component]
pub async fn order_totals(
    lines: Vec<TotalLine>,
    #[default] action: Option<String>,
    #[default]
    #[into]
    action_label: String,
    #[default] mut attrs: Attributes,
    #[default] child: View,
) -> Result {
    let action_label = if action_label.is_empty() {
        "Continue".to_owned()
    } else {
        action_label
    };

    view! {
        <section
            data-topcoat-block="order-totals"
            class=(class!("w-full", attrs.remove("class")))
            (attrs)
        >
            card(
                card_header(card_title("Order summary"))
                card_content(
                    <dl class="grid gap-3">
                        for line in lines {
                            <div class=(class!("flex items-baseline justify-between gap-4", line.tone.classes()))>
                                <dt>(line.label)</dt>
                                <dd class="text-right">(line.value)</dd>
                            </div>
                        }
                    </dl>
                )
                match action {
                    Some(href) => card_footer(
                        attrs: attributes! { class="grid gap-3 border-t border-border pt-5" },
                        (child)
                        <a
                            href=(href)
                            class=(button_variants(ButtonVariant::Primary, ButtonSize::Lg))
                        >
                            (action_label)
                        </a>
                    ),
                    None => card_footer(
                        attrs: attributes! { class="border-t border-border pt-5 empty:hidden" },
                        (child)
                    ),
                }
            )
        </section>
    }
}
