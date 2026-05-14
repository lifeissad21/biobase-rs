#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegisteredCallMethod {
    pub name: &'static str,
    pub arity: usize,
}

pub const REGISTERED_CALL_METHODS: [RegisteredCallMethod; 5] = [
    RegisteredCallMethod {
        name: "rowQ",
        arity: 2,
    },
    RegisteredCallMethod {
        name: "rowMedians",
        arity: 4,
    },
    RegisteredCallMethod {
        name: "unsafe_set_slot",
        arity: 3,
    },
    RegisteredCallMethod {
        name: "lc_prefix",
        arity: 2,
    },
    RegisteredCallMethod {
        name: "sublist_extract",
        arity: 4,
    },
];

pub fn registered_call_methods() -> &'static [RegisteredCallMethod] {
    &REGISTERED_CALL_METHODS
}
