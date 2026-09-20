use std::fmt;

pub const GID_PREFIX: &str = "gid://shopify/";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gid<'a> {
    kind: &'a str,
    id: &'a str,
}

impl<'a> Gid<'a> {
    pub const ORDER: &'static str = "Order";

    pub fn of(kind: &'a str, id: &'a str) -> Self {
        Self {
            kind,
            id: Self::id_of(id),
        }
    }

    pub fn id_of(value: &str) -> &str {
        match value.strip_prefix(GID_PREFIX) {
            Some(rest) => rest
                .split('?')
                .next()
                .unwrap_or(rest)
                .rsplit('/')
                .next()
                .unwrap_or(rest),
            None => value,
        }
    }

    pub fn kind(&self) -> &'a str {
        self.kind
    }

    pub fn id(&self) -> &'a str {
        self.id
    }
}

impl fmt::Display for Gid<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{GID_PREFIX}{}/{}", self.kind, self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_a_gid_from_a_bare_id() {
        assert_eq!(
            Gid::of(Gid::ORDER, "1001").to_string(),
            "gid://shopify/Order/1001"
        );
    }

    #[test]
    fn building_from_a_gid_is_idempotent() {
        let once = Gid::of(Gid::ORDER, "1001").to_string();
        assert_eq!(Gid::of(Gid::ORDER, &once).to_string(), once);
    }

    #[test]
    fn id_of_strips_the_prefix_and_the_type() {
        assert_eq!(Gid::id_of("gid://shopify/Order/1001"), "1001");
        assert_eq!(Gid::id_of("gid://shopify/ProductVariant/11"), "11");
    }

    #[test]
    fn id_of_drops_query_parameters() {
        assert_eq!(
            Gid::id_of("gid://shopify/Order/1001?namespace=custom"),
            "1001"
        );
    }

    #[test]
    fn id_of_leaves_a_value_that_is_not_a_gid_alone() {
        assert_eq!(Gid::id_of("1001"), "1001");
        assert_eq!(Gid::id_of(""), "");
        assert_eq!(Gid::id_of("shopify/Order/1001"), "shopify/Order/1001");
    }

    #[test]
    fn parts_are_readable_back() {
        let gid = Gid::of(Gid::ORDER, "gid://shopify/Order/1001");
        assert_eq!(gid.kind(), "Order");
        assert_eq!(gid.id(), "1001");
    }
}
