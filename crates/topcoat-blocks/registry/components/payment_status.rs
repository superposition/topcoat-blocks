use super::{
    badge::{BadgeVariant, badge},
    button::{ButtonSize, ButtonVariant, button_variants},
    card::{card, card_content, card_footer, card_header, card_title},
    progress::progress,
};
use topcoat::{
    Result,
    view::{Attributes, attributes, class, component, view},
};

/// Portable payment lifecycle state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PaymentState {
    #[default]
    AwaitingWallet,
    AwaitingTransfer,
    Confirming,
    Paid,
    Expired,
    Canceled,
    ManualReview,
}

impl PaymentState {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AwaitingWallet => "awaiting-wallet",
            Self::AwaitingTransfer => "awaiting-transfer",
            Self::Confirming => "confirming",
            Self::Paid => "paid",
            Self::Expired => "expired",
            Self::Canceled => "canceled",
            Self::ManualReview => "manual-review",
        }
    }

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::AwaitingWallet => "Connect wallet",
            Self::AwaitingTransfer => "Awaiting transfer",
            Self::Confirming => "Confirming",
            Self::Paid => "Paid",
            Self::Expired => "Expired",
            Self::Canceled => "Canceled",
            Self::ManualReview => "Manual review",
        }
    }

    const fn badge_variant(self) -> BadgeVariant {
        match self {
            Self::Paid => BadgeVariant::Secondary,
            Self::Expired | Self::Canceled => BadgeVariant::Destructive,
            Self::ManualReview | Self::Confirming => BadgeVariant::Outline,
            Self::AwaitingWallet | Self::AwaitingTransfer => BadgeVariant::Primary,
        }
    }
}

/// Host-owned content rendered by [`payment_status`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PaymentStatus {
    pub state: PaymentState,
    pub title: String,
    pub message: String,
    pub amount: String,
    pub network: String,
    pub recipient: Option<String>,
    pub transaction_href: Option<String>,
    pub transaction_label: Option<String>,
    pub recovery_href: Option<String>,
}

/// Status surface for wallet connection, transfer, confirmation, and recovery.
///
/// The host owns all state transitions and transaction verification.
#[component]
pub async fn payment_status(
    status: PaymentStatus,
    #[default] mut attrs: Attributes,
) -> Result {
    let state = status.state.as_str();

    view! {
        <section
            data-topcoat-block="payment-status"
            data-state=(state)
            aria-live="polite"
            class=(class!("w-full", attrs.remove("class")))
            (attrs)
        >
            card(
                card_header(
                    <div class="flex items-start justify-between gap-4">
                        <div>
                            <p class="text-sm text-muted-foreground">(status.network.as_str())</p>
                            card_title(attrs: attributes! { class="mt-1" }, (status.title.as_str()))
                        </div>
                        badge(
                            variant: status.state.badge_variant(),
                            (status.state.label())
                        )
                    </div>
                )
                card_content(
                    attrs: attributes! { class="grid gap-5" },
                    match status.state {
                        PaymentState::Confirming => progress(
                            value: 65.0,
                            attrs: attributes! {
                                aria-label="Payment confirmation progress"
                            }
                        ),
                        _ => {},
                    }
                    <p class="leading-7 text-muted-foreground">(status.message.as_str())</p>
                    <dl class="grid gap-3 rounded-lg bg-muted/50 p-4 text-sm">
                        <div class="flex justify-between gap-4">
                            <dt class="text-muted-foreground">"Amount"</dt>
                            <dd class="font-semibold">(status.amount.as_str())</dd>
                        </div>
                        match status.recipient.as_deref() {
                            Some(recipient) => <div class="grid gap-1">
                                <dt class="text-muted-foreground">"Recipient"</dt>
                                <dd class="break-all font-mono text-xs">(recipient)</dd>
                            </div>,
                            None => {},
                        }
                    </dl>
                )
                match (
                    status.transaction_href.as_deref(),
                    status.recovery_href.as_deref(),
                ) {
                    (None, None) => {},
                    (transaction_href, recovery_href) => card_footer(
                        attrs: attributes! { class="flex flex-wrap gap-3 border-t border-border pt-5" },
                        match transaction_href {
                            Some(href) => <a
                                href=(href)
                                rel="external noreferrer"
                                class=(button_variants(ButtonVariant::Outline, ButtonSize::Md))
                            >
                                (status.transaction_label.as_deref().unwrap_or("View transaction"))
                            </a>,
                            None => {},
                        }
                        match recovery_href {
                            Some(href) => <a
                                href=(href)
                                class=(button_variants(ButtonVariant::Ghost, ButtonSize::Md))
                            >
                                "Recover payment"
                            </a>,
                            None => {},
                        }
                    ),
                }
            )
        </section>
    }
}
