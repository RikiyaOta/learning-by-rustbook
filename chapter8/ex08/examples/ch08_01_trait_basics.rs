use std::f32::consts::PI;

#[derive(Debug)]
struct CartesianCoord {
    x: f64,
    y: f64,
}

#[derive(Debug)]
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

// へ〜。struct で定義した自分の型だけじゃなくって、
// 既存の型にも trait を実装できるんだ。
impl Coordinates for (f64, f64) {
    fn to_cartesian(self) -> CartesianCoord {
        CartesianCoord {
            x: self.0,
            y: self.1,
        }
    }

    fn from_cartesian(cart: CartesianCoord) -> Self {
        (cart.x, cart.y)
    }
}

fn main() {
    // (f64, f64)に実装したメソッドを使ってみる。
    let point = (1.0, 1.0);
    let c = point.to_cartesian();

    println!("c: {:?}", c);

    let p = PolarCoord::from_cartesian(c);
    println!("p: {:?}", p);
}
