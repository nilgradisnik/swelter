use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::gio;
use gtk::glib;
use glib::{ParamFlags, ParamSpec, ParamSpecInt, ParamSpecFloat, Value};

use once_cell::sync::Lazy;
use std::cell::Cell;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct SwelterChartArea {
        pub tick: Cell<i32>,
        pub temperature: Cell<f32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SwelterChartArea {
        const NAME: &'static str = "SwelterChartArea";
        type Type = super::SwelterChartArea;
        type ParentType = gtk::DrawingArea;
    }

    impl ObjectImpl for SwelterChartArea {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            let matrix = cairo::Matrix::new(1.0, 0.0, 0.0, -1.0, 0.0, 20.0);

            obj.set_draw_func(move |area, ctx, _, _| {
                let tick = area.property::<i32>("tick");
                let temperature = area.property::<f32>("temperature");

                println!("Tick {:?} Temp: {:?}", tick, temperature);

                let context = ctx.to_owned();
                context.set_matrix(matrix);
                context.scale(20.0, 20.0);
                context.set_source_rgb(1.0, 0.0, 0.0);
                context.set_line_width(0.1);

                let x = 10.0 - (tick as f32 / 10.0);
                let y = temperature / 100.0;

                context.move_to(x.into(), 0.0);
                context.line_to(x.into(), y.into());

                context.stroke().expect("Unable to draw on context");
            });
        }

        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![ParamSpecInt::new(
                    "tick",
                    "tick",
                    "tick",
                    i32::MIN,
                    i32::MAX,
                    0,
                    ParamFlags::READWRITE,
                ), ParamSpecFloat::new(
                    "temperature",
                    "temperature",
                    "temperature",
                    f32::MIN,
                    f32::MAX,
                    0.0,
                    ParamFlags::READWRITE,
                )]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "tick" => {
                    let value = value.get().expect("The value needs to be of type `i32`.");
                    self.tick.replace(value);
                }
                "temperature" => {
                    let value = value.get().expect("The value needs to be of type `f32`.");
                    self.temperature.replace(value);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "tick" => self.tick.get().to_value(),
                "temperature" => self.temperature.get().to_value(),
                _ => unimplemented!(),
            }
        }
    }

    impl WidgetImpl for SwelterChartArea {}

    impl DrawingAreaImpl for SwelterChartArea {
        fn resize(&self, _drawing_area: &Self::Type, _width: i32, _height: i32) {}
    }
}

glib::wrapper! {
    pub struct SwelterChartArea(ObjectSubclass<imp::SwelterChartArea>)
        @extends gtk::Widget, gtk::DrawingArea,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl SwelterChartArea {
    pub fn new() -> Self {
        glib::Object::new(&[])
            .expect("Failed to create SwelterChartArea")
    }

    pub fn update_temperature(&self, temperature: f32) {
        let mut tick = self.property::<i32>("tick");
        tick += 1;
        self.set_property("tick", &tick);

        self.set_property("temperature", &temperature);

        self.queue_draw();
    }
}   
