use leptos::prelude::*;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(
    class = "relative w-full rounded-lg border p-4 [&>svg~*]:pl-7 [&>svg+div]:translate-y-[-3px] [&>svg]:absolute [&>svg]:left-4 [&>svg]:top-4 [&>svg]:text-foreground"
)]
pub struct AlertClass {
    pub variant: AlertVariant,
}

#[derive(TwVariant)]
pub enum AlertVariant {
    #[tw(default, class = "bg-background text-foreground")]
    Default,
    #[tw(
        class = "border-destructive/50 text-destructive dark:border-destructive [&>svg]:text-destructive"
    )]
    Destructive,
}

#[component]
pub fn Alert(
    #[prop(into, optional)] variant: Signal<AlertVariant>,
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| {
        AlertClass {
            variant: variant.get(),
        }
        .with_class(class.get())
    });

    view! {
        <div class=class>
            {children()}
        </div>
    }
}

#[derive(TwClass)]
#[tw(class = "mb-1 font-medium leading-none tracking-tight")]
pub struct AlertTitleClass {}

#[component]
pub fn AlertTitle(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| AlertTitleClass {}.with_class(class.get()));

    view! {
        <h5 class=class>
            {children()}
        </h5>
    }
}

#[derive(TwClass)]
#[tw(class = "text-sm [&_p]:leading-relaxed")]
pub struct AlertDescriptionClass {}

#[component]
pub fn AlertDescription(
    #[prop(into, optional)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| AlertDescriptionClass {}.with_class(class.get()));

    view! {
        <div class=class>
            {children()}
        </div>
    }
}
