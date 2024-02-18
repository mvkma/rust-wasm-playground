use std::f64::consts::PI;

use crate::{FlowFieldFunction, Vec2D};

pub const FIELDS: [FlowFieldFunction; 10] = [
    FlowFieldFunction {
        name: "{ cos(2*pi*r^2), sin(2*pi*r^2) }",
        func: |x: f64, y: f64| Vec2D {
            x: ((x.powf(2.0) + y.powf(2.0)) * PI * 2.0).cos(),
            y: ((x.powf(2.0) + y.powf(2.0)) * PI * 2.0).sin(),
        },
    },
    FlowFieldFunction {
        name: "{ cos(2*pi*r^2), sin(2*pi*r^2) }",
        func: |x: f64, y: f64| Vec2D {
            x: ((x.powf(2.0) + y.powf(2.0) - x * y) * PI * 2.0).cos(),
            y: ((x.powf(2.0) + y.powf(2.0) - x * y) * PI * 2.0).sin(),
        },
    },
    FlowFieldFunction {
        name: "{ cos(2*pi*log(x*y)), sin(2*pi*sin(x*y)) }",
        func: |x: f64, y: f64| Vec2D {
            x: ((x * y).ln() * PI * 2.0).cos(),
            y: ((x * y).ln() * PI * 2.0).sin(),
        },
    },
    FlowFieldFunction {
        name: "{ cos(2*pi*log(r^2)), sin(2*pi*log(r^2)) }",
        func: |x: f64, y: f64| Vec2D {
            x: ((x.powf(2.0) + y.powf(2.0)).ln() * PI * 2.0).cos(),
            y: ((x.powf(2.0) + y.powf(2.0)).ln() * PI * 2.0).sin(),
        },
    },
    FlowFieldFunction {
        name: "{ cos(2*pi*x/y), sin(2*pi*x/y) }",
        func: |x: f64, y: f64| Vec2D {
            x: ((x / y) * PI * 2.0).cos(),
            y: ((x / y) * PI * 2.0).sin(),
        },
    },
    FlowFieldFunction {
        name: "{ cos(2*pi*exp(-(3*x)^2 + (3*y)^2)), sin(2*pi*exp(-(3*x)^2 + (3*y)^2)) }",
        func: |x: f64, y: f64| {
            let x = 3.0 * x;
            let y = 3.0 * y;
            let s = -x.powf(2.0) + y.powf(2.0);
            Vec2D {
                x: (s.exp() * PI * 2.0).cos(),
                y: (s.exp() * PI * 2.0).sin(),
            }
        },
    },
    FlowFieldFunction {
        name: "grad((x^2 + y^2)^2 - 1.5 * (x^2 + y^2))",
        func: |x: f64, y: f64| {
            // TODO: Proper scaling function and translations
            let x = (x - 0.5) * 2.5;
            let y = (y - 0.5) * 2.5;
            Vec2D {
                x: 4.0 * x * (x.powf(2.0) + y.powf(2.0)) - 3.0 * x,
                y: 4.0 * y * (x.powf(2.0) + y.powf(2.0)) - 3.0 * y,
            }
        },
    },
    FlowFieldFunction {
        name: "grad(sin(x^2 + y^2))",
        func: |x: f64, y: f64| {
            // TODO: Proper scaling function and translations
            let x = (x - 0.5) * 10.0;
            let y = (y - 0.5) * 10.0;
            Vec2D {
                x: 2.0 * x * (x.powf(2.0) + y.powf(2.0)).cos(),
                y: 2.0 * y * (x.powf(2.0) + y.powf(2.0)).cos(),
            }
        },
    },
    FlowFieldFunction {
        name: "grad(sin(x^2 + y^2) * exp(-(x^2 + y^2) / 2 / pi))",
        func: |x: f64, y: f64| {
            // TODO: Proper scaling function and translations
            let x = (x - 0.5) * 8.0;
            let y = (y - 0.5) * 8.0;
            let rr = x.powf(2.0) + y.powf(2.0);
            Vec2D {
                x: (2.0 * PI * rr.cos() - rr.sin()) * x * (-rr / PI / 2.0).exp() / PI,
                y: (2.0 * PI * rr.cos() - rr.sin()) * y * (-rr / PI / 2.0).exp() / PI,
            }
        },
    },
    FlowFieldFunction {
        name: "grad(log(x^2 / y^2 + 1))",
        func: |x: f64, y: f64| {
            // TODO: Proper scaling function and translations
            let x = (x - 0.5) * 6.0;
            let y = (y - 0.5) * 6.0;
            let rr = x.powf(2.0) + y.powf(2.0);
            Vec2D {
                x: 2.0 * x / rr,
                y: -2.0 * x.powf(2.0) / rr / y,
            }
        },
    },
];
