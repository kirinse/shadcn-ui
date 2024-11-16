use leptos::{
    attr::any_attribute::AnyAttribute,
    ev::MouseEvent,
    html::{self, ElementType, HtmlElement},
    prelude::*,
};
use leptos_node_ref::AnyNodeRef;
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
    pub node_ref: AnyNodeRef,
    pub id: MaybeProp<String>,
    pub class: Signal<String>,
    pub style: MaybeProp<String>,
    pub disabled: bool,
    pub onclick: Option<Callback<MouseEvent>>,
}

impl ButtonChildProps {
    pub fn render(self, children: Option<Children>) -> AnyView {
        view! {
            <button
                node_ref={self.node_ref}
                id=move || self.id.get()
                class=self.class
                style=move || self.style.get()
                disabled=self.disabled
                on:click={move |event| if let Some(onclick) = self.onclick {
                    onclick.run(event)
                }}
            >
                {children.map(|children| children())}
            </button>
        }
        .into_any()
    }
}

#[derive(Default)]
pub struct Dynamic {
    // attributes: Vec<AnyAttribute>,
}

impl Dynamic {
    pub fn render<E, At, Ch, F: Fn() -> HtmlElement<E, At, Ch>>(self, tag: F) -> AnyView {
        View::new(tag()).into_any()
    }
}

#[component]
pub fn Button(
    #[prop(into, optional)] variant: Signal<ButtonVariant>,
    #[prop(into, optional)] size: Signal<ButtonSize>,
    #[prop(into, optional)] disabled: bool,
    #[prop(into, optional)] on_click: Option<Callback<MouseEvent>>,
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] dynamic: Dynamic,
    // #[prop(into, optional)] as_child: Option<Callback<ButtonChildProps, AnyView>>,
    #[prop(into, optional)] as_child: Option<Callback<Dynamic, AnyView>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class = Memo::new(move |_| {
        ButtonClass {
            variant: variant.get(),
            size: size.get(),
        }
        .with_class(class.get().unwrap_or_default())
    });

    // let child_props = ButtonChildProps {
    //     node_ref,
    //     id,
    //     class: class.into(),
    //     style,
    //     disabled,
    //     onclick: on_click,
    // };

    // if let Some(as_child) = as_child.as_ref() {
    //     as_child.run(child_props)
    // } else {
    //     child_props.render(children)
    // }

    if let Some(as_child) = as_child.as_ref() {
        as_child.run(dynamic)
    } else {
        dynamic.render(leptos::tachys::html::element::button)
    }
}

#[derive(Clone)]
pub struct TooltipChildProps {
    pub node_ref: AnyNodeRef,
    pub id: MaybeProp<String>,
    pub class: MaybeProp<String>,
    pub style: MaybeProp<String>,
}

#[component]
fn Tooltip(
    #[prop(into, optional)] node_ref: AnyNodeRef,
    #[prop(into, optional)] id: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] style: MaybeProp<String>,
    #[prop(into, optional)] dynamic: Dynamic,
    // #[prop(into)] as_child: Callback<TooltipChildProps, AnyView>,
    #[prop(into)] as_child: Callback<Dynamic, AnyView>,
) -> impl IntoView {
    // let child_props = TooltipChildProps {
    //     node_ref,
    //     id,
    //     class,
    //     style,
    // };

    // as_child.run(child_props)

    as_child.run(dynamic)
}

#[component]
fn App() -> impl IntoView {
    let test = view! {
        <button />
    }
    .into_any();

    view! {
        // <Tooltip
        //     as_child=Callback::new(|TooltipChildProps { node_ref, .. }| view! {
        //         <Button node_ref={node_ref}>Test</Button>
        //     }.into_any())
        // />
        <Tooltip
            as_child=Callback::new(|dynamic| view! {
                <Button dynamic={dynamic}>Test</Button>
            }.into_any())
        />

        // <Button as_child=Callback::new(|ButtonChildProps { node_ref, ..}| view! {
        //     <a node_ref={node_ref} href="#">Test</a>
        // }.into_any()) />
    }
}
