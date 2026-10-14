#[derive(Debug, Clone)]
pub struct Estadistica{
    pub prob_acierto: f32,
    pub prob_bloqueo: f32,
    pub postura_defensiva: bool,
}

impl Default for Estadistica{
    fn default() -> Self {
        Self {
            prob_acierto: 0.75,
            prob_bloqueo: 0.20,
            postura_defensiva: false,
        }
    }
}
