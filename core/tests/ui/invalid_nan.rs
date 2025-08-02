use core::time_temperature_curve::impl_polyline_const::ImplPolylineConst;

const _INVALID_NAN: ImplPolylineConst<2> = ImplPolylineConst::from_array([
    (0.0, f64::NAN),
    (1.0, 2.0),
]);

fn main() {}
