pub mod custom_syntax;
pub mod default;
pub mod ders;
pub mod mut_path;
pub mod ser;
pub mod wrap;

use crate::*;

pub trait TokenStreamExt {
    fn into_pretty(self) -> Res<String>;
}

impl TokenStreamExt for TokenStream {
    #[doc(alias = "display")]
    fn into_pretty(self) -> Res<String> {
        Ok(prettyplease::unparse(&syn::parse2(self)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::*;

    #[test]
    fn test_expr_vec() {
        assert_eq!(
            vec![true, false],
            <Vec<bool> as DeRs<Expr>>::de(&syn::parse_str::<Expr>("[true, false]").unwrap())
                .unwrap()
        );

        assert_eq!(
            vec![vec![true, false],],
            <Vec<Vec<bool>> as DeRs<Expr>>::de(&syn::parse_str::<Expr>("[[true, false]]").unwrap())
                .unwrap()
        );
    }

    #[test]
    fn test_expr_option_vec() {
        assert_eq!(
            Some(vec![true, false]),
            DeRs::de(&syn::parse_str::<Expr>("Some([true, false])").unwrap()).unwrap()
        );
    }

    #[test]
    fn test_expr_box_vec() {
        assert_eq!(
            Box::new(vec![true, false]),
            DeRs::de(&syn::parse_str::<Expr>("[true, false]").unwrap()).unwrap()
        );
    }

    #[test]
    fn test_expr_option() {
        dbg!(&syn::parse_str::<Expr>("None").unwrap());

        assert_eq!(
            Some(123u8),
            DeRs::de(&syn::parse_str::<Expr>("Some(123)").unwrap()).unwrap()
        );
        assert_eq!(
            None::<u8>,
            DeRs::de(&syn::parse_str::<Expr>("None").unwrap()).unwrap()
        );
    }

    #[test]
    fn test_expr_tuple() {
        assert_eq!(
            (1_u8, true, 0.1234_f64),
            DeRs::de(&syn::parse_str::<Expr>("(1, true, 0.1234)").unwrap()).unwrap()
        );
    }

    #[test]
    fn test_expr_bool() {
        assert_eq!(
            true,
            DeRs::de(&syn::parse_str::<Expr>("true").unwrap()).unwrap()
        );
        assert_eq!(
            false,
            DeRs::de(&syn::parse_str::<Expr>("false").unwrap()).unwrap()
        );
    }

    #[test]
    fn test_expr_float() {
        assert_eq!(
            f32::MIN,
            DeRs::de(&syn::parse_str::<Expr>("-3.4028235e38").unwrap()).unwrap()
        );
        assert_eq!(
            f32::MAX,
            DeRs::de(&syn::parse_str::<Expr>("3.4028235e38").unwrap()).unwrap()
        );
    }

    #[test]
    fn test_expr_int() {
        assert_eq!(
            i32::min_value(),
            DeRs::de(&syn::parse_str::<Expr>("-2147483648").unwrap()).unwrap()
        );
    }
}
