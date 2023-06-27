macro_rules! boolean_action {
    ($action: ident, $bool: expr) => {
        match $action {
            True => $bool = true,
            False => $bool = false,
            Toggle => $bool = !$bool,
        }
    };
}

pub(super) use boolean_action;
