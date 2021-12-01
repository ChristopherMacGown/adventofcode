type Module = usize;
type Cost = usize;
type Spaceship = [Module; 100];

trait RocketEquation {
    fn fuel_cost(&self) -> Cost;
    fn total_fuel_cost(&self) -> Cost;
}

impl RocketEquation for Module {
    fn fuel_cost(&self) -> Cost {
        (self / 3).saturating_sub(2)
    }

    fn total_fuel_cost(&self) -> Cost {
        // This includes the fuel cost with the cost of the fuel itself.
        let mut net = self.fuel_cost();
        let mut partial = (net as Module).fuel_cost();
        while partial != 0 {
            net += partial;
            partial = (partial as Module).fuel_cost();
        }
        net
    }
}

impl RocketEquation for Spaceship {
    fn fuel_cost(&self) -> Cost {
        self.iter().fold(0, |total, m| total + m.fuel_cost())
    }

    fn total_fuel_cost(&self) -> Cost {
        self.iter().fold(0, |total, m| total + m.total_fuel_cost())
    }
}

#[cfg(test)]
mod tests {
    use super::{Module, RocketEquation, Spaceship};

    const MODULES: Spaceship = [
        106985, 113927, 107457, 106171, 69124, 59906, 66420, 149336, 73783, 120127, 139486, 108698,
        104091, 103032, 108609, 136293, 144735, 55381, 98823, 103981, 140684, 114482, 133925,
        111247, 110833, 92252, 87396, 79730, 61395, 82572, 72403, 140763, 57088, 63457, 65523,
        50148, 134758, 93447, 85513, 132927, 139159, 141579, 94444, 56997, 137128, 107930, 67607,
        108837, 120206, 79441, 99839, 137404, 140502, 67274, 108736, 97302, 76561, 107804, 134306,
        52820, 89632, 101473, 65001, 57399, 82858, 60577, 82043, 144783, 101606, 138900, 68246,
        118774, 129919, 99394, 80009, 107404, 121503, 119232, 108157, 117965, 112025, 139205,
        126336, 143985, 58894, 93020, 136732, 100535, 144090, 134414, 109049, 105714, 111654,
        50677, 77622, 53398, 133851, 71166, 115935, 94067,
    ];

    #[test]
    fn test_fuel_costs() {
        assert_eq!((12 as Module).fuel_cost(), 2);
        assert_eq!((12 as Module).total_fuel_cost(), 2);
        assert_eq!((14 as Module).fuel_cost(), 2);
        assert_eq!((14 as Module).total_fuel_cost(), 2);
        assert_eq!((1969 as Module).fuel_cost(), 654);
        assert_eq!((1969 as Module).total_fuel_cost(), 966);
        assert_eq!((100756 as Module).fuel_cost(), 33583);
        assert_eq!((100756 as Module).total_fuel_cost(), 50346);

        assert_eq!(MODULES.fuel_cost(), 3389778);
        assert_eq!(MODULES.total_fuel_cost(), 5081802);
    }
}
