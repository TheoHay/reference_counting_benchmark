pub const TEST_TEXT_BIG: &'static str = r#"Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Nullam lacinia a elit eu efficitur.
Morbi feugiat, sem et consectetur pharetra, metus orci maximus enim, non tristique elit augue ut ex.
Praesent tempus arcu ut lacus molestie, a scelerisque libero bibendum.
Curabitur iaculis felis id magna scelerisque, vel ultrices felis laoreet.
Phasellus sollicitudin leo at risus.
Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Nullam lacinia a elit eu efficitur.
Morbi feugiat, sem et consectetur pharetra, metus orci maximus enim, non tristique elit augue ut ex.
Praesent tempus arcu ut lacus molestie, a scelerisque libero bibendum.
Curabitur iaculis felis id magna scelerisque, vel ultrices felis laoreet.
Phasellus sollicitudin leo at risus.
Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Nullam lacinia a elit eu efficitur.
Morbi feugiat, sem et consectetur pharetra, metus orci maximus enim, non tristique elit augue ut ex.
Praesent tempus arcu ut lacus molestie, a scelerisque libero bibendum.
Curabitur iaculis felis id magna scelerisque, vel ultrices felis laoreet.
Phasellus sollicitudin leo at risus."#;

pub const TEST_TEXT_MEDIUM: &'static str = r#"Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Nullam lacinia a elit eu efficitur.
Morbi feugiat, sem et consectetur pharetra, metus orci maximus enim, non tristique elit augue ut ex.
Praesent tempus arcu ut lacus molestie, a scelerisque libero bibendum.
Curabitur iaculis felis id magna scelerisque, vel ultrices felis laoreet.
Phasellus sollicitudin leo at risus."#;

pub const TEST_TEXT_SMALL: &'static str = r#"Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Nullam lacinia a elit eu efficitur."#;

pub enum ObjectSize {
    Small,
    Medium,
    Big,
}

impl ObjectSize {
    pub fn get_str(&self) -> &'static str {
        match self {
            &ObjectSize::Small => TEST_TEXT_SMALL,
            &ObjectSize::Medium => TEST_TEXT_MEDIUM,
            &ObjectSize::Big => TEST_TEXT_BIG,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            &ObjectSize::Small => "Small Object",
            &ObjectSize::Medium => "Medium Object",
            &ObjectSize::Big => "Big Object",
        }
    }
}
