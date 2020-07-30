use druid::kurbo::PathEl;
use druid::widget::prelude::*;
use druid::widget::{Align, Button, Flex, Label, Padding, Radio, Slider, WidgetExt};
use druid::{Affine, Point, Rect, TimerToken};
use druid::{AppLauncher, Data, Lens, PlatformError, Widget, WindowDesc};
use std::time::Duration;

const PI: f64 = 3.1415926535;

#[derive(Data, Clone, Lens, Default)]
struct Model {
    slider: f64,
    radio: bool,
    #[data(same_fn = "PartialEq::eq")]
    time: Duration,
}

struct CanvasWidget {
    timer: TimerToken,
}

impl Widget<Model> for CanvasWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Model, _env: &Env) {
        match event {
            Event::WindowConnected => {
                ctx.request_paint();
                let next_frame = Duration::from_millis(20);
                self.timer = ctx.request_timer(next_frame);
            }
            Event::Timer(token) => {
                if *token == self.timer {
                    ctx.request_paint();
                    let next_frame = Duration::from_millis(20);
                    self.timer = ctx.request_timer(next_frame);
                    data.time += next_frame;
                }
            }
            _ => {}
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &Model,
        _env: &Env,
    ) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &Model, _data: &Model, _env: &Env) {
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &Model,
        _env: &Env,
    ) -> Size {
        // BoxConstraints are passed by the parent widget.
        // This method can return any Size within those constraints:
        //
        // To check if a dimension is infinite or not (e.g. scrolling):
        // bc.is_width_bounded() / bc.is_height_bounded()
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Model, env: &Env) {
        let size = ctx.size();
        let rect = Rect::from_origin_size(Point::ORIGIN, size);

        ctx.clip(rect);

        ctx.fill(rect, &env.get(druid::theme::BACKGROUND_DARK));

        let sin_wave: Vec<_> = (0..(size.width as usize))
            .map(|x| {
                let shifted_x = x + (data.time.as_millis() / 20) as usize;
                let norm_x = shifted_x as f64 / size.width; // from 0..1
                let calc_x = norm_x * data.slider * 2. * PI; // 0..(2pi * slider)
                let y = match data.radio {
                    true => calc_x.sin(),
                    false => calc_x.tan(),
                };
                PathEl::LineTo(Point::new(x as f64, y * size.height / 4.))
            })
            .collect();

        ctx.transform(Affine::translate((0., size.height / 2.)));

        ctx.stroke(&sin_wave[..], &env.get(druid::theme::PRIMARY_DARK), 2.0);

        ctx.restore().unwrap();
    }
}

fn build_ui() -> impl Widget<Model> {
    Padding::new(
        (10.0, 20.0),
        Flex::column()
            .with_flex_child(
                Flex::row()
                    .with_flex_child(
                        Flex::column()
                            .with_flex_child(Label::new("Hello, World"), 1.0)
                            .with_spacer(20.0)
                            .with_flex_child(Button::new("A Button!"), 1.0)
                            .with_child(Slider::new().with_range(0., 10.).lens(Model::slider)),
                        1.0,
                    )
                    .with_flex_child(
                        Align::right(
                            Flex::column()
                                .with_flex_child(Label::new("OK"), 1.0)
                                .with_spacer(10.0)
                                .with_flex_child(Radio::new("T", true).lens(Model::radio), 1.0)
                                .with_flex_child(Radio::new("F", false).lens(Model::radio), 1.0),
                        ),
                        1.0,
                    ),
                0.5,
            )
            .with_flex_child(
                CanvasWidget {
                    timer: TimerToken::INVALID,
                },
                3.0,
            )
            .with_flex_child(
                Align::centered(Label::dynamic(|data: &Model, _| {
                    format!("Freq: {}, Time: {}s", data.slider, data.time.as_secs())
                })),
                0.5,
            ),
    )
}

fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(WindowDesc::new(build_ui).window_size((800., 800.)))
        .launch(Model::default())?;
    Ok(())
}
