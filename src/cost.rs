/// Container for attibutes calculation in central panel
///
/// Mainly for debug purposes
pub struct CostView<'a> {
    calc: AttributeCalc<'a>,
}

/// Calculations for character stats, attibutes, cots, etc.
///
/// Mutably borrows the [`Character`](crate::data::Character),
/// [`AttributeCalc`] must not outlive this borrow.
pub struct AttributeCalc<'a> {
    character: &'a mut crate::data::Character,
}

impl<'a> CostView<'a> {
    /// Create new cost view container
    ///
    /// # Arguments
    ///
    /// * `character` – Used for calculations
    pub fn new(character: &'a mut crate::data::Character) -> Self {
        Self {
            calc: AttributeCalc::new(character),
        }
    }

    /// Add UI elements
    ///
    /// Mostly debugging
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.label(format!("Start: {}", self.calc.start_ap()));

        debug_ui!(ui, self.calc.spezies_debug());

        debug_ui!(ui, self.calc.le_debug());

        debug_ui!(ui, self.calc.sk_debug());
        debug_ui!(ui, self.calc.zk_debug());

        debug_ui!(ui, self.calc.ausweichen_debug());
    }
}

/// Generator macro for ui_*attribute*() like methods.
///
/// Methods return a closure which adds a corresponding [`Decimal`](crate::widgets::decimal::Decimal) widget to the ui.
///
/// # Synopsys
///
/// `ui_attr!(fun, attr);`
///
/// * `fun`  – the name of the method to generate
/// * `attr` – the attribute in [`character`](crate::cost::AttributeCalc::character).[`attributes`](crate::data::Character::attributes) to link to the widget
///
/// # Example
///
/// ```
/// // This will create an method `ui_mu`
/// // which returns a closure adding a widget to ui
/// // that updates the `mu` attribute of the character
/// ui_attr!(ui_mu, mu);   
/// ```
macro_rules! ui_attr {
    ($fun: ident, $attr: ident) => {
        pub fn $fun(&mut self) -> impl egui::Widget + '_ {
            |ui: &mut egui::Ui| {
                ui.add(crate::widgets::decimal::Decimal::decimal(
                    &mut self.character.attributes.$attr,
                    0..=self
                        .character
                        .erfahrungsgrad
                        .erfahrungsgrad()
                        .eigenschaft_max,
                ))
            }
        }
    };
}

impl<'a> AttributeCalc<'a> {
    pub fn new(character: &'a mut crate::data::Character) -> Self {
        Self { character }
    }

    pub fn start_ap(&self) -> u16 {
        self.base_ap()
    }

    pub fn spezies_debug(&self) -> String {
        format!(
            "Spezies: {} LE {} SK {} ZK {} COST {}\nStart - Spezies = {}",
            self.base_name(),
            self.base_le(),
            self.base_sk(),
            self.base_zk(),
            self.base_cost(),
            self.base_ap() - self.base_cost() as u16
        )
    }

    pub fn effective_le(&self) -> u8 {
        self.character.identity.species.le() + 2 * self.character.attributes.ko
    }

    pub fn le_debug(&self) -> String {
        format!(
            "Effective LE (LE({}) + 2 * KO({})): {}",
            self.base_le(),
            self.character.attributes.ko,
            self.effective_le(),
        )
    }

    pub fn effective_sk(&self) -> i16 {
        let part_sum = (self.character.attributes.mu
            + self.character.attributes.kl
            + self.character.attributes.r#in) as i16;

        self.character.identity.species.sk() as i16 + round_div(part_sum, 6)
    }

    pub fn sk_debug(&self) -> String {
        format!(
            "Effective SK (SK({}) + (MU({}) + KL({}) + IN({})) / 6): {}",
            self.base_sk(),
            self.character.attributes.mu,
            self.character.attributes.kl,
            self.character.attributes.r#in,
            self.effective_sk()
        )
    }

    pub fn effective_zk(&self) -> i16 {
        let part_sum = (self.character.attributes.ko
            + self.character.attributes.ko
            + self.character.attributes.kk) as i16;

        self.character.identity.species.zk() as i16 + round_div(part_sum, 6)
    }

    pub fn zk_debug(&self) -> String {
        format!(
            "Effective ZK (ZK({}) + (KO({}) + KO({}) + KK({})) / 6): {}",
            self.base_sk(),
            self.character.attributes.ko,
            self.character.attributes.ko,
            self.character.attributes.kk,
            self.effective_zk()
        )
    }

    pub fn ausweichen(&self) -> i16 {
        round_div(self.character.attributes.ge as i16, 2)
    }

    pub fn ausweichen_debug(&self) -> String {
        format!(
            "Ausweichen (GE({}) / 2): {}",
            self.character.attributes.ge,
            self.ausweichen()
        )
    }

    ui_attr!(ui_mu, mu);
    ui_attr!(ui_kl, kl);
    ui_attr!(ui_in, r#in);
    ui_attr!(ui_ch, ch);
    ui_attr!(ui_ff, ff);
    ui_attr!(ui_ge, ge);
    ui_attr!(ui_ko, ko);
    ui_attr!(ui_kk, kk);

    fn base_ap(&self) -> u16 {
        self.character.erfahrungsgrad.erfahrungsgrad().ap_konto
    }

    fn base_name(&self) -> &'static str {
        self.character.identity.species.name()
    }

    fn base_le(&self) -> u8 {
        self.character.identity.species.le()
    }

    fn base_sk(&self) -> i8 {
        self.character.identity.species.sk()
    }

    fn base_zk(&self) -> i8 {
        self.character.identity.species.zk()
    }

    /// Kosten der Spezies an Sich
    fn base_cost(&self) -> u8 {
        self.character.identity.species.cost()
    }
}

const fn round_div(a: i16, b: i16) -> i16 {
    let round_pad = if a % b < b / 2 { 0 } else { 1 };
    a / b + round_pad
}
