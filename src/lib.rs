pub struct XAppFoundation;
impl XAppFoundation {
    pub fn get_standard_app_id_prefix() -> &'static str { "org.sigmaos" }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_xapp_prefix() {
        assert_eq!(XAppFoundation::get_standard_app_id_prefix(), "org.sigmaos");
    }
}
