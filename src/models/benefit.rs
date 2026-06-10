//! Modèle représentant un bénéfice développemental.

#[derive(Clone)]
pub struct Benefit {
    pub key: &'static str,
    pub label: &'static str,
    pub summary: &'static str,
    pub detail: &'static str,
    pub source: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benefit_instantiation() {
        let benefit = Benefit {
            key: "resilience",
            label: "Resilience",
            summary: "Sum",
            detail: "Det",
            source: "Src",
        };
        assert_eq!(benefit.key, "resilience");
    }
}
