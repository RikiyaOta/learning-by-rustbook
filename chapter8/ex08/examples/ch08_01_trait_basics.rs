use std::f64::consts::PI;

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
                theta: if cart.y >= 0.0 { PI / 2.0 } else { -PI / 2.0 },
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

// Pattern1
fn print_point<P: Coordinates>(point: P) {
    let p = point.to_cartesian();
    println!("({}, {})", p.x, p.y);
}

//// Pattern2
//fn print_point2<P>(point: P) where P: Coordinates {
//    let p = point.to_cartesian();
//    println!("({}, {})", p.x, p.y);
//}
//
//// Pattern3
//fn print_point3(point: impl Coordinates) {
//    let p = point.to_cartesian();
//    println!("({}, {})", p.x, p.y);
//}

// 複数の　trait boundary をつけることも可能
fn as_cartesian<P: Coordinates + Clone>(point: &P) -> CartesianCoord {
    point.clone().to_cartesian()
}

// もうちょっと複雑なトレイト境界になると、冗長な where 記法を使わないといけない。
// むしろ、これくらい柔軟にかけるのがすごい。
fn make_point<T>(x: T, y: T) -> CartesianCoord
where
    (T, T): Coordinates,
{
    (x, y).to_cartesian()
}

trait ConvertTo<Output> {
    fn convert(&self) -> Output;
}

// なるほど。
// built in type に対してもトレイト境界を設定することで、トレイトを実装していることを要請できるのか。
// わざわざ struct とか書かなくてもいいのは柔軟性がある気がする。
fn to<T>(i: i32) -> T
where
    i32: ConvertTo<T>,
{
    i.convert()
}

fn main() {
    // (f64, f64)に実装したメソッドを使ってみる。
    let point = (1.0, 1.0);
    let c = point.to_cartesian();

    println!("c: {:?}", c);

    let p = PolarCoord::from_cartesian(c);
    println!("p: {:?}", p);

    // Coordinates trait を実装しているのでコンパイルとおる。
    print_point((0.0, 1.0));
    print_point(PolarCoord {
        r: 1.0,
        theta: PI / 2.0,
    });

    // print_point("string"); // &str は通らない。Coordinates を実装してないから。
}
