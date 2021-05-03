use relm_derive::widget;

#[widget]
impl Widget for Foo {
    fn model() {}

    fn update() {}

    view! {
        gtk::Window {}
    }
}

fn main() {}
