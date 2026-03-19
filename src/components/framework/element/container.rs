use std::cmp::PartialEq;
use crate::components::context::context;
use crate::components::framework::animation::AnimationRegistry;
use crate::components::framework::ui_traits::{TickResult, UIHandler};
use crate::components::framework::event::{Event, EventResult, RenderPass};
use crate::components::framework::layout::{LayoutContext, LayoutDirection, LayoutEvent, Sizing};
use crate::components::render::stack::State;
use crate::components::spatial::vec2::Vec2;
use crate::components::spatial::vec4::Vec4;

#[macro_export]
macro_rules! container {
    (
        layout: { $($layout:tt)* },
        $( $expr:expr ),* $(,)?
    ) => {{
        let mut container = Container::new(
            LayoutContext {
                $($layout)*
                ..Default::default()
            }
        );

        container.bounds.set_wh(container.layout.pref_size);

        $(
            container.add($expr);
        )*

        container
    }};

    (
        $( $expr:expr ),* $(,)?
    ) => {{
        let mut container = Container::new(LayoutContext::default());

        $(
            container.add($expr);
        )*

        container
    }};
}

pub struct Container {
    pub bounds: Vec4,
    children: Vec<Box<dyn UIHandler>>,
    pub layout: LayoutContext
}

impl Container {
    pub fn new(layout: LayoutContext) -> Self {
        let mut c = Container {
            bounds: Vec4::zero().clone(),
            children: vec![],
            layout,
        };

        c.bounds.set_wh(c.layout.pref_size);

        match c.layout.size_behavior {
            (Sizing::Fixed(v), _) => {
                c.bounds.set_width(v);
            },
            (_, Sizing::Fixed(v)) => {
                c.bounds.set_height(v);
            },
            _ => {}
        }

        c
    }

    unsafe fn grow_size(&mut self, direction: &LayoutDirection, v: f32) -> EventResult {
        let self_behavior = match direction {
            LayoutDirection::Horizontal =>  &self.layout.size_behavior.0,
            LayoutDirection::Vertical =>  &self.layout.size_behavior.1,
        };
        match self_behavior {
            Sizing::Grow => {self.bounds.expand_direction(direction, v);}
            _ => {}
        }

        let margin_total = match direction {
            LayoutDirection::Horizontal => self.layout.margin.left() + self.layout.margin.right(),
            LayoutDirection::Vertical => self.layout.margin.top() + self.layout.margin.bottom(),
        };
        if &self.layout.direction != direction {
            for c in &mut self.children {
                let size = c.bounds().direction_size(direction);
                let behavior = match direction {
                    LayoutDirection::Horizontal => c.layout_context().size_behavior.0,
                    LayoutDirection::Vertical => c.layout_context().size_behavior.1
                };
                match behavior {
                    Sizing::Grow => {
                        let remaining_size = self.bounds.direction_size(direction) - margin_total - c.bounds().direction_size(direction);
                        match c.handle(&Event::Layout(LayoutEvent::grow_direction(direction, remaining_size))) {
                            EventResult::LayoutError => {
                                println!("LAYOUT ERR 3");
                                break;
                            }
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
            return EventResult::Ok;
        }
        loop {
            let mut max = 0.;
            let mut min = f32::MAX;
            let mut second_min = f32::MAX;
            let mut total_size = 0.;
            let mut smallest_index = 0;
            let mut second_smallest_index = 0;
            let mut growables = 0;
            let mut index = 0;
            for c in &mut self.children {
                let size = c.bounds().direction_size(direction);
                let behavior = match direction {
                    LayoutDirection::Horizontal => c.layout_context().size_behavior.0,
                    LayoutDirection::Vertical => c.layout_context().size_behavior.1
                };
                match behavior {
                    Sizing::Grow => {
                        if size <= min {
                            second_smallest_index = smallest_index;
                            second_min = min;

                            smallest_index = index;
                            min = size;
                        }
                        growables += 1;
                    },
                    _ => {}
                }
                match behavior {
                    Sizing::Fixed(_) | Sizing::Grow => {
                        total_size += size;
                        max = size.max(max);
                    }
                    _ => {}
                }
                index += 1;
            }
            let space_used =
                if &self.layout.direction == direction {
                    total_size + (self.children.len().max(1) - 1) as f32 * self.layout.spacing.direction(direction)
                } else {
                    max
                };
            let mut remaining_size = self.bounds.direction_size(direction) - margin_total - space_used;

            if remaining_size == 0. || growables == 0 {
                return EventResult::Ok;
            }

            if second_min > min && growables > 2 {
                match self.children.get_mut(smallest_index).unwrap().handle(&Event::Layout(LayoutEvent::grow_direction(direction, (second_min - min).min(remaining_size)))) {
                    EventResult::LayoutError => {
                        println!("LAYOUT ERR 1");
                        panic!()
                        // break;
                    }
                    _ => {}
                }
            } else {
                let len = self.children.len();
                for c in &mut self.children {
                    let behavior = match direction {
                        LayoutDirection::Horizontal => c.layout_context().size_behavior.0,
                        LayoutDirection::Vertical => c.layout_context().size_behavior.1
                    };

                    if matches!(behavior, Sizing::Grow) {
                        match c.handle(&Event::Layout(LayoutEvent::grow_direction(direction, remaining_size / len as f32))) {
                            EventResult::LayoutError => {
                                println!("LAYOUT ERR 2");
                                panic!()
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    pub unsafe fn fit(&mut self, direction: &LayoutDirection, event: &Event) -> EventResult {
        let margin_total = match direction {
            LayoutDirection::Horizontal => self.layout.margin.left() + self.layout.margin.right(),
            LayoutDirection::Vertical => self.layout.margin.top() + self.layout.margin.bottom(),
        };

        self.bounds.set_direction_size(direction, margin_total);
        let mut max = 0.;
        let mut total = (self.children.len().max(1) - 1) as f32 * self.layout.spacing.direction(direction);
        for c in &mut self.children {
            c.handle(event);
            let size = c.bounds().direction_size(direction);
            max = size.max(max);
            total += size;
        }
        if &self.layout.direction == direction {
            self.bounds.expand_direction(direction, total);
        } else {
            self.bounds.set_direction_size(direction, max + margin_total);
        }
        self.bounds.set_direction_size(direction, self.bounds.direction_size(direction).max(*self.layout.min_size.direction(direction)));

        EventResult::Ok
    }

    pub fn add<H: UIHandler + 'static>(&mut self, child: H) {
        self.children.push(Box::new(child));
    }
}

impl UIHandler for Container {
    unsafe fn handle(&mut self, event: &Event) -> EventResult {
        let debug = true;
        if debug {
            self.bounds.debug_draw(self.layout.debug_color);
        }
        // Translate child positions, which also offsets mouse correctly
        context().renderer().stack().push(State::Translate(self.bounds().x(), self.bounds().y()));
        // println!("transled to {:?}", self.bounds);
        let mut result = EventResult::Ok;
        for c in &mut self.children {
            match event {
                Event::PostRender => {
                    match c.animations() {
                        None => {}
                        Some(mut reg) => { reg.post(); }
                    }
                }
                _ => {}
            }
            
            match c.handle(event) {
                EventResult::Ok => {},
                r => { result = r }
            }
        }
        context().renderer().stack().pop();

        result = match event {
            Event::Layout(stage) => {

                match stage {
                    LayoutEvent::FitWidth => {
                        self.fit(&LayoutDirection::Horizontal, event)
                    }
                    LayoutEvent::FitHeight => {
                        self.fit(&LayoutDirection::Vertical, event)
                    }
                    LayoutEvent::GrowWidth(v) => {
                        self.grow_size(&LayoutDirection::Horizontal, *v)
                    }
                    LayoutEvent::GrowHeight(v) => {
                        self.grow_size(&LayoutDirection::Vertical, *v)
                    }
                    LayoutEvent::Position(pos) => {
                        self.bounds.set_pos(*pos);
                        let mut current_pos = Vec2::new(self.layout.margin.x, self.layout.margin.y);
                        for c in &mut self.children {
                            c.handle(&Event::Layout(LayoutEvent::Position(current_pos)));
                            current_pos.add_direction(&self.layout.direction, *self.layout.spacing.direction(&self.layout.direction) + c.bounds().direction_size(&self.layout.direction));
                        }
                        EventResult::Ok
                    }
                    _ => result
                }
            }
            _ => result
        };

        result
    }

    unsafe fn tick(&mut self, render_pass: &RenderPass) -> TickResult {
        for c in &mut self.children {
            let r = c.tick(render_pass);
            if !r.is_valid() {
                return r
            }
        }
        TickResult::Valid
    }

    fn animations(&mut self) -> Option<AnimationRegistry> {
        None
    }

    fn bounds(&self) -> Vec4 {
        self.bounds
    }

    fn layout_context(&self) -> LayoutContext {
        self.layout.clone()
    }
}