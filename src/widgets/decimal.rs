use std::ops::RangeBounds;

/// A Decimal is always limited.
/// The default limit is the types upper and lower bounds.
// Currently implemented for 16 bit integers
pub struct Decimal<'a, T: eframe::emath::Numeric> {
    decimal: LimitedDecimal<'a, T>,
}

pub struct LimitedDecimal<'a, T: egui::emath::Numeric> {
    value: &'a mut T,
    limits: Limits<T>,
}

pub struct Limits<T> {
    upper: T,
    lower: T,
}

// impl<'a> From<&'a mut u16> for LimitedDecimal<'a, u16> {
//     fn from(value: &'a mut u16) -> Self {
//         LimitedDecimal {
//             value,
//             limits: Limits {
//                 upper: u16::MAX,
//                 lower: u16::MIN,
//             },
//         }
//     }
// }

impl<'a, T> From<&'a mut T> for LimitedDecimal<'a, T>
where
    T: egui::emath::Numeric,
{
    fn from(value: &'a mut T) -> Self {
        Self {
            value,
            limits: Limits {
                upper: egui::emath::Numeric::MAX,
                lower: egui::emath::Numeric::MIN,
            },
        }
    }
}

impl<'a, T, N> From<(&'a mut N, T)> for LimitedDecimal<'a, N>
where
    N: egui::emath::Numeric,
    T: RangeBounds<N>,
{
    fn from((value, bounds): (&'a mut N, T)) -> Self {
        LimitedDecimal {
            value,
            limits: Limits {
                upper: match bounds.end_bound() {
                    std::ops::Bound::Included(&upper) => upper,
                    // The upper bound is excluded, so take the next lower integer
                    // overflow would panic anyways …
                    std::ops::Bound::Excluded(&upper) => {
                        // if upper > egui::emath::Numeric::MIN {
                        //let Some(true) = upper.checked_sub(1).map(|u| value.1.contains(&u)) {
                        egui::emath::Numeric::from_f64(upper.to_f64() - 1.0)
                        // } else {
                        // panic!(
                        // "Upper End of Range was Excluded but next integer was out of Range!"
                        // )
                        // }
                    }
                    std::ops::Bound::Unbounded => egui::emath::Numeric::MAX,
                },
                lower: match bounds.start_bound() {
                    std::ops::Bound::Included(&lower) => lower,
                    std::ops::Bound::Excluded(&lower) => {
                        // if lower < egui::emath::Numeric::MAX {
                        //let Some(true) = lower.checked_add(1).map(|u| value.1.contains(&u)) {
                        egui::emath::Numeric::from_f64(lower.to_f64() + 1.0)
                        // } else {
                        // panic!(
                        // "Lower End of Range was Excluded but next integer was out of Range!"
                        // )
                        // }
                    }
                    std::ops::Bound::Unbounded => egui::emath::Numeric::MIN,
                },
            },
        }
    }
}

// impl<'a> From<&'a mut i16> for LimitedDecimal<'a, i16> {
//     fn from(value: &'a mut i16) -> Self {
//         LimitedDecimal {
//             value,
//             limits: Limits {
//                 upper: i16::MAX,
//                 lower: i16::MIN,
//             },
//         }
//     }
// }
// impl<'a, T> From<(&'a mut i16, T)> for LimitedDecimal<'a, i16>
// where
//     T: RangeBounds<i16>,
// {
//     fn from(value: (&'a mut i16, T)) -> Self {
//         LimitedDecimal {
//             value: value.0,
//             limits: Limits {
//                 upper: match value.1.end_bound() {
//                     std::ops::Bound::Included(&upper) => upper,
//                     std::ops::Bound::Excluded(&upper) => {
//                         if let Some(true) = upper.checked_sub(1).map(|u| value.1.contains(&u)) {
//                             upper - 1
//                         } else {
//                             panic!(
//                                 "Upper End of Range was Excluded but next integer was out of Range!"
//                             )
//                         }
//                     }
//                     std::ops::Bound::Unbounded => i16::MAX,
//                 },
//                 lower: match value.1.start_bound() {
//                     std::ops::Bound::Included(&lower) => lower,
//                     std::ops::Bound::Excluded(&lower) => {
//                         if let Some(true) = lower.checked_add(1).map(|u| value.1.contains(&u)) {
//                             lower + 1
//                         } else {
//                             panic!(
//                                 "Lower End of Range was Excluded but next integer was out of Range!"
//                             )
//                         }
//                     }
//                     std::ops::Bound::Unbounded => i16::MIN,
//                 },
//             },
//         }
//     }
// }

impl<'a, T: eframe::emath::Numeric> std::ops::AddAssign<f64> for LimitedDecimal<'a, T> {
    fn add_assign(&mut self, rhs: f64) {
        *self.value = egui::emath::Numeric::from_f64(self.value.to_f64() + rhs)
    }
}
impl<'a, T: eframe::emath::Numeric> std::ops::SubAssign<f64> for LimitedDecimal<'a, T> {
    fn sub_assign(&mut self, rhs: f64) {
        *self.value = egui::emath::Numeric::from_f64(self.value.to_f64() - rhs)
    }
}

impl<'a, T: eframe::emath::Numeric> Decimal<'a, T> {
    pub fn new<U: RangeBounds<T>>(value: &'a mut T, range: U) -> Self
    where
        (&'a mut T, U): Into<LimitedDecimal<'a, T>>,
    {
        Self {
            decimal: (value, range).into(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) -> egui::InnerResponse<()> {
        let desired_size = ui.spacing().interact_size * egui::vec2(1.0, 3.0);
        let layout = egui::Layout::top_down_justified(egui::Align::Center);
        let respone = ui.allocate_ui_with_layout(desired_size, layout, move |ui| {
            let button_increment = egui::Button::new("⏶").frame(false);
            if ui
                .add_enabled(
                    *self.decimal.value < self.decimal.limits.upper,
                    button_increment,
                )
                .clicked()
            {
                self.decimal += 1.0
            }

            let limits = &self.decimal.limits;
            let value_display =
                egui::DragValue::new(self.decimal.value).clamp_range(limits.lower..=limits.upper);
            ui.add(value_display);

            let button_decrement = egui::Button::new("⏷").frame(false);
            if ui
                .add_enabled(
                    *self.decimal.value > self.decimal.limits.lower,
                    button_decrement,
                )
                .clicked()
            {
                self.decimal -= 1.0
            }
        });
        respone
    }

    pub fn decimal<U: RangeBounds<T> + 'a>(value: &'a mut T, range: U) -> impl egui::Widget + '_ {
        move |ui: &mut egui::Ui| Self::new(value, range).show(ui).response
    }
}
