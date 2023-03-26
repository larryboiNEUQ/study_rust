trait TrafficSignal {
    fn duration(&self) -> u8;
}

enum SignalColor {
    Red,
    Yellow,
    Green,
}

struct RedSignal;
struct YellowSignal;
struct GreenSignal;

impl TrafficSignal for RedSignal {
    fn duration(&self) -> u8 {
        10 // 返回红灯亮起时持续的时间
    }
}

impl TrafficSignal for YellowSignal {
    fn duration(&self) -> u8 {
        3 // 返回黄灯亮起时持续的时间
    }
}

impl TrafficSignal for GreenSignal {
    fn duration(&self) -> u8 {
        15 // 返回绿灯亮起时持续的时间
    }
}

fn main() {
    let red_signal = RedSignal;
    let yellow_signal = YellowSignal;
    let green_signal = GreenSignal;

    println!("红灯持续时间：{}秒", red_signal.duration());
    println!("黄灯持续时间：{}秒", yellow_signal.duration());
    println!("绿灯持续时间：{}秒", green_signal.duration());
}
