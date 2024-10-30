use leptos::{ev::MouseEvent, html, prelude::*};
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(
    class = "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0"
)]
pub struct ButtonClass {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
}

#[derive(PartialEq, TwVariant)]
pub enum ButtonVariant {
    #[tw(
        default,
        class = "bg-primary text-primary-foreground hover:bg-primary/90"
    )]
    Default,
    #[tw(class = "bg-destructive text-destructive-foreground hover:bg-destructive/90")]
    Destructive,
    #[tw(class = "border border-input bg-background hover:bg-accent hover:text-accent-foreground")]
    Outline,
    #[tw(class = "bg-secondary text-secondary-foreground hover:bg-secondary/80")]
    Secondary,
    #[tw(class = "hover:bg-accent hover:text-accent-foreground")]
    Ghost,
    #[tw(class = "text-primary underline-offset-4 hover:underline")]
    Link,
}

#[derive(PartialEq, TwVariant)]
pub enum ButtonSize {
    #[tw(default, class = "h-10 px-4 py-2")]
    Default,
    #[tw(class = "h-9 rounded-md px-3")]
    Sm,
    #[tw(class = "h-11 rounded-md px-8")]
    Lg,
    #[tw(class = "h-10 w-10")]
    Icon,
}

#[derive(Clone)]
pub struct ButtonChildProps {
    pub node_ref: NodeRef<html::Button>,
    pub id: MaybeProp<String>,
    pub class: Memo<String>,
    pub style: MaybeProp<String>,
    pub disabled: bool,
    pub onclick: Option<Callback<MouseEvent>>,
}

impl ButtonChildProps {
    pub fn render(self, children: Children) -> AnyView {
        view! {
            <button
                node_ref={self.node_ref}
                // TODO: figure out how to pass the Signal/Memo directly
                // id=move || self.id.get()
                // id=self.id
                class=self.class
                style=move || self.style.get()
                disabled=self.disabled
                // TODO
                // on:click={self.onclick.unwrap()}
            >
                {children()}
            </button>
        }
        .into_any()
    }
}

#[component]
pub fn Button(
    #[prop(into, optional)] variant: Signal<ButtonVariant>,
    #[prop(into, optional)] size: Signal<ButtonSize>,
    #[prop(into, optional)] disabled: bool,
    #[prop(into, optional)] on_click: Option<Callback<MouseEvent>>,
    #[prop(into, optional)] node_ref: NodeRef<html::Button>,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] as_child: Option<Callback<ButtonChildProps, AnyView>>,
    children: Children,
) -> impl IntoView {
    let class = Memo::new(move |_| {
        ButtonClass {
            variant: variant.get(),
            size: size.get(),
        }
        .with_class(class.get().unwrap_or_default())
    });

    let child_props = ButtonChildProps {
        node_ref,
        id,
        class,
        style,
        disabled,
        onclick: on_click,
    };

    if let Some(as_child) = as_child.as_ref() {
        as_child.run(child_props)
    } else {
        child_props.render(children)
    }
}
