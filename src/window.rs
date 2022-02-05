use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, pango, CompositeTemplate};

use glib_macros::closure_local;
use glib::subclass::Signal;

use once_cell::sync::Lazy;

pub const SENSORS_UPDATED_SIGNAL: &str = "sensors-updated";

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/org/gnome/Swelter/window.ui")]
    pub struct SwelterWindow {
        #[template_child]
        pub header_bar: TemplateChild<gtk::HeaderBar>,
        #[template_child]
        pub grid: TemplateChild<gtk::Grid>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SwelterWindow {
        const NAME: &'static str = "SwelterWindow";
        type Type = super::SwelterWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for SwelterWindow {
        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder(
                    SENSORS_UPDATED_SIGNAL,
                    &[<Vec<String>>::static_type().into()],
                    <()>::static_type().into(),
                )
                .build()]
            });
            SIGNALS.as_ref()
        }

        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            self.grid.set_row_spacing(10);
            self.grid.set_column_spacing(10);

            let grid = self.grid.clone();

            obj.connect_closure(
                SENSORS_UPDATED_SIGNAL, false,
                closure_local!(@strong grid => move |_obj: &Self::Type, values: Vec<String>| {
                    println!("Values: {:?}", values);

                    let index: i32 = values[0].parse().unwrap();
                    let name = &values[1];
                    let temperature = &format!("{}°C", values[2]);

                    if let Some(child) = grid.child_at(1, index) {
                        let label = child.dynamic_cast::<gtk::Label>();
                        label.unwrap().set_text(name);
                    } else {
                        let mut font_description = pango::FontDescription::new();
                        font_description.set_weight(pango::Weight::Bold);

                        let attributes = pango::AttrList::new();
                        attributes.insert(pango::AttrFontDesc::new(&font_description));

                        let label = gtk::Label::builder()
                            .label(name)
                            .halign(gtk::Align::Start)
                            .attributes(&attributes)
                            .build();

                        grid.attach(&label, 1, index, 1, 1);
                    }

                    if let Some(child) = grid.child_at(2, index) {
                        let label = child.dynamic_cast::<gtk::Label>();
                        label.unwrap().set_text(temperature);
                    } else {
                        let label = gtk::Label::builder()
                            .label(temperature)
                            .halign(gtk::Align::Start)
                            .build();

                        grid.attach(&label, 2, index, 1, 1);
                    }
                }),
            );
        }
    }

    impl WidgetImpl for SwelterWindow {}
    impl WindowImpl for SwelterWindow {}
    impl ApplicationWindowImpl for SwelterWindow {}
}

glib::wrapper! {
    pub struct SwelterWindow(ObjectSubclass<imp::SwelterWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl SwelterWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::new(&[("application", application)])
            .expect("Failed to create SwelterWindow")
    }
}
