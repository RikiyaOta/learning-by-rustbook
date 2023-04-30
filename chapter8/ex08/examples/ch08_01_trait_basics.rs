use std::f32::consts::PI;

struct CartesianCoord {
    x: f64,
    y: f64,
}

struct PolarCoord {
    r: f64,
    theta: f64,
}

trait Coordinates {
    fn to_cartesian(self) -> CartesianCoord;
    fn from_cartesian(cart: CartesianCoord) -> Self;
}

impl Coordinates for CartesianCoord {
    fn to_cartesian(self) -> CartesianCoord {
        self
    }

    fn from_cartesian(cart: CartesianCoord) -> Self {
        cart
    }
}

impl Coordinates for PolarCoord {
    fn to_cartesian(self) -> CartesianCoord {
        CartesianCoord {
            x: self.r * self.theta.cos(),
            y: self.r * self.theta.sin(),
        }
    }

    fn from_cartesian(cart: CartesianCoord) -> Self {
        // NOTE: carx.x == 0 の場合の考慮が漏れてるのでは？
        // PolarCoord { r: (cart.x * cart.x + cart.y * cart.y).sqrt(), theta: (cart.y / cart.x).atan() }

        // 改良する:
        if cart.x < 1e-10 {
            PolarCoord {
                r: cart.y.abs(),
                theta: if cart.y >= 0.0 {
                    (PI / 2.0).into()
                } else {
                    (-PI / 2.0).into()
                },
            }
        } else {
            PolarCoord {
                r: (cart.x * cart.x + cart.y * cart.y).sqrt(),
                theta: (cart.y / cart.x).atan(),
            }
        }
    }
}

fn main() {
    println!("Hello!");
}
