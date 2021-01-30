// Copyright 2019 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A radio button widget.

use std::time::Duration;

use druid::kurbo::Circle;
use druid::widget::prelude::*;
use druid::widget::{CrossAxisAlignment, Flex, Label, LabelText};
use druid::{Data, LinearGradient, TimerToken, UnitPoint, theme};

use crate::env::key::PRIMARY;

const DEFAULT_RADIO_RADIUS: f64 = 9.0;
const INNER_CIRCLE_RADIUS: f64 = 5.0;
const EXTRA_PADDING: f64 = 20.0;
/// A group of radio buttons
#[derive(Debug, Clone)]
pub struct RadioGroup;

impl RadioGroup {
    /// Given a vector of `(label_text, enum_variant)` tuples, create a group of Radio buttons
    pub fn new<T: Data + PartialEq>(
        variants: impl IntoIterator<Item = (impl Into<LabelText<T>> + 'static, T)>,
    ) -> impl Widget<T> {
        let mut col = Flex::column().cross_axis_alignment(CrossAxisAlignment::Start);
        let mut is_first = true;
        for (label, variant) in variants.into_iter() {
            if !is_first {
                col.add_default_spacer();
            }
            let radio = Radio::new(label, variant);
            col.add_child(radio);
            is_first = false;
        }
        col
    }
}

/// A single radio button
pub struct Radio<T> {
    variant: T,
    child_label: Label<T>,
    expanding_time: Option<TimerToken>,
    expanding_radius: Option<f64>,
}

impl<T: Data> Radio<T> {
    /// Create a lone Radio button from label text and an enum variant
    pub fn new(label: impl Into<LabelText<T>>, variant: T) -> Radio<T> {
        Radio {
            variant,
            child_label: Label::new(label),
            expanding_time: None,
            expanding_radius: None,
        }
    }
}

impl<T: Data + PartialEq> Widget<T> for Radio<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::MouseDown(_) => {
                ctx.set_active(true);
                ctx.request_paint();
            }
            Event::MouseUp(_) => {
                if ctx.is_active() {
                    ctx.set_active(false);
                    if ctx.is_hot() {
                        *data = self.variant.clone();
                        ctx.request_paint();
                        ctx.request_anim_frame();
                        let timer = ctx.request_timer(Duration::from_millis(200));
                        self.expanding_time = Some(timer);
                        self.expanding_radius = Some(10.0);
                    }
                }
            }
            Event::AnimFrame(f) if self.expanding_time.is_some() => {
                let fraction_passed = (*f as f64) / 1.0e6 / 200.0;
                let to_expand_radii = (env.get(theme::BASIC_WIDGET_HEIGHT) + EXTRA_PADDING) / 2.0;
                *self.expanding_radius.as_mut().unwrap() += fraction_passed * (to_expand_radii - 10.0);
                ctx.request_anim_frame();
                ctx.request_paint();
            }
            Event::Timer(t) if self.expanding_time == Some(*t) => {
                self.expanding_time = None;
                self.expanding_radius = None;
                ctx.request_paint();
            },
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.child_label.lifecycle(ctx, event, data, env);
        if let LifeCycle::HotChanged(_) = event {
            ctx.request_paint();
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.child_label.update(ctx, old_data, data, env);
        if !old_data.same(data) {
            ctx.request_paint();
        }
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        bc.debug_check("Radio");

        let radio_size = env.get(theme::BASIC_WIDGET_HEIGHT) + EXTRA_PADDING;
        let label_size = self.child_label.layout(ctx, &bc, data, env);

        let desired_size = Size::new(
            radio_size + label_size.width,
            radio_size.max(label_size.height),
        );
        let our_size = bc.constrain(desired_size);
        let baseline = self.child_label.baseline_offset() + (our_size.height - label_size.height);
        ctx.set_baseline_offset(baseline);
        our_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let size = env.get(theme::BASIC_WIDGET_HEIGHT) + EXTRA_PADDING;
        let x_padding = env.get(theme::WIDGET_CONTROL_COMPONENT_PADDING);
        let is_selected = *data == self.variant;
        
        let circle = Circle::new((size / 2., size / 2.), DEFAULT_RADIO_RADIUS);
        let radius = size / 2.;

        let border_color = if is_selected {
            env.get(PRIMARY)
        } else {
            env.get(theme::BORDER_LIGHT)
        };


        // circles
        if let Some(r) = self.expanding_radius {
            let circle = Circle::new((radius, radius), r);
            ctx.fill(circle, &border_color.clone().with_alpha(0.3));
        }
        else if ctx.is_focused() {
            let circle = Circle::new((radius, radius), radius);
            ctx.fill(circle, &border_color.clone());
        } else if ctx.is_hot() {
            let circle = Circle::new((radius, radius), radius);
            ctx.fill(circle, &border_color.clone().with_alpha(0.05));
        }



        ctx.stroke(circle, &border_color, 2.);

        // Check if data enum matches our variant
        if is_selected {
            let inner_circle = Circle::new((size / 2., size / 2.), INNER_CIRCLE_RADIUS);

            ctx.fill(inner_circle, &border_color);
        }

        // Paint the text label
        // 2.0 is empherical determined constant
        self.child_label.draw_at(ctx, (size, EXTRA_PADDING / 2.0 - 1.0));
    }
}
