use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::gio;
use gtk::glib;
use gtk::glib::clone;

use std::cell::RefCell;

mod imp {
    use super::*;

    pub struct SwelterChartArea {
        pub temperatures: RefCell<Vec<f32>>,
    }

    impl Default for SwelterChartArea {
        fn default() -> SwelterChartArea {
            SwelterChartArea {
                temperatures: RefCell::new(vec![]),
            }
        }
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

            obj.set_draw_func(clone!(@weak obj => move |_, ctx, _, _| {
                let imp = obj.imp();

                let temperatures = imp.temperatures.borrow();

                println!("Temps: {:?}", temperatures);

                let context = ctx.to_owned();

                // Invert coordinates and scale
                context.set_matrix(matrix);
                context.scale(20.0, 20.0);

                // Draw background
                context.set_source_rgb(1.0, 1.0, 1.0);
                context.paint().expect("Unable to draw on context");

                // Draw chart
                for (index, temperature) in temperatures.iter().enumerate() {
                    context.set_source_rgb(0.8, 0.1, 0.1);
                    context.set_line_width(0.1);

                    let x = 19.5 - (index as f32 / 10.0);
                    let y = temperature / 100.0;

                    context.move_to(x.into(), 0.0);
                    context.line_to(x.into(), y.into());

                    context.stroke().expect("Unable to draw on context");
                }
            }));
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
        let imp = self.imp();

        let mut temperatures = imp.temperatures.borrow_mut();
        temperatures.push(temperature);

        self.queue_draw();
    }
}

impl Default for SwelterChartArea {
    fn default() -> Self {
        SwelterChartArea::new()
    }
}

