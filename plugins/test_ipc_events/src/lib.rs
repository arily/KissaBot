pub enum CustomEvents {
    CustomEvent1,
    CustomEvent2(&'static str), // Change the lifetime to 'static
    CustomEvent3(&'static str, &'static str, &'static str), // Also change here
}
