// use crate::error_format::ErrorInfo;
use crate::data::primitive::{PrimitiveArray, PrimitiveBoolean, PrimitiveObject, PrimitiveString};
use crate::data::Literal;

fn get_accept(lit: &Literal) -> Option<&Literal> {
    let val = lit
        .primitive
        .as_any()
        .downcast_ref::<PrimitiveObject>()?
        .value
        .get("accepts")?;
    Some(val)
}

// TODO: change when exec
fn contains(array_lit: &Literal, key: &Literal) -> Literal {
    let key_string = key.primitive.as_any().downcast_ref::<PrimitiveString>();
    match (
        array_lit
            .primitive
            .as_any()
            .downcast_ref::<PrimitiveArray>(),
        key_string,
    ) {
        (Some(array), None) => {
            PrimitiveBoolean::get_literal(array.value.contains(key), array_lit.interval.to_owned())
        }
        (Some(array), Some(string)) => {
            for elem in array.value.iter() {
                match elem.primitive.as_any().downcast_ref::<PrimitiveString>() {
                    Some(val)
                        if val.value.to_ascii_lowercase() == string.value.to_ascii_lowercase() =>
                    {
                        return PrimitiveBoolean::get_literal(true, elem.interval.to_owned())
                    }
                    _ => continue,
                }
            }
            PrimitiveBoolean::get_literal(false, array_lit.interval.to_owned())
        }
        (None, ..) => PrimitiveBoolean::get_literal(false, array_lit.interval.to_owned()),
    }
}

pub fn match_obj(lit1: &Literal, lit2: &Literal) -> Literal {
    match (&lit1.content_type, &lit2.content_type) {
        (b1, b2) if (b1 == "button" || b1 == "object") && (b2 == "button" || b2 == "object") => {
            match (get_accept(lit1), get_accept(lit2)) {
                (Some(l1), Some(l2)) => match_obj(l1, l2),
                (_, _) => PrimitiveBoolean::get_literal(false, lit1.interval.to_owned()),
            }
        }

        (.., button) if (button == "button" || button == "object") => match get_accept(lit2) {
            Some(l2) => match_obj(lit1, l2),
            None => PrimitiveBoolean::get_literal(false, lit1.interval.to_owned()),
        },
        (button, ..) if (button == "button" || button == "object") => match get_accept(lit1) {
            Some(l1) => match_obj(l1, lit2),
            None => PrimitiveBoolean::get_literal(false, lit1.interval.to_owned()),
        },

        (array1, array2) if array1 == "array" && array2 == "array" => {
            PrimitiveBoolean::get_literal(lit1 == lit2, lit1.interval.to_owned())
        }
        (.., array) if array == "array" => contains(lit2, lit1),
        (array, ..) if array == "array" => contains(lit1, lit2),
        (..) => PrimitiveBoolean::get_literal(
            lit1.primitive == lit2.primitive.to_owned(),
            lit1.interval,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::primitive::array::PrimitiveArray;
    use crate::data::primitive::string::PrimitiveString;
    use crate::data::{ast::Interval, tokens::*};
    use crate::interpreter::builtins::buttons::button;
    use std::collections::HashMap;

    fn gen_inter() -> Interval {
        Interval { line: 0, column: 0 }
    }

    fn gen_button(name: &str) -> Literal {
        let mut map = HashMap::new();
        let interval = gen_inter();

        map.insert(
            DEFAULT.to_owned(),
            PrimitiveString::get_literal(name, interval),
        );

        match button(map, interval) {
            Ok(lit) => lit,
            Err(..) => panic!("gen button error"),
        }
    }

    fn gen_button_multi_accept(name: &str) -> Literal {
        let mut map = HashMap::new();
        let interval = gen_inter();

        map.insert(
            DEFAULT.to_owned(),
            PrimitiveString::get_literal(name, interval),
        );
        map.insert(
            "accepts".to_owned(),
            PrimitiveArray::get_literal(
                &vec![
                    PrimitiveString::get_literal("toto", interval),
                    PrimitiveString::get_literal("plop", interval),
                    PrimitiveString::get_literal("TEST", interval),
                ],
                gen_inter(),
            ),
        );

        match button(map, interval) {
            Ok(lit) => lit,
            Err(..) => panic!("gen button error"),
        }
    }

    fn match_lit_ok(lit1: &Literal, lit2: &Literal) {
        match match_obj(&lit1, &lit2).primitive.as_bool() {
            boolean if boolean => {}
            _ => panic!("\n\nlit1: {:?}\n\n lit2: {:?}\n", lit1, lit2),
        }
    }

    fn match_lit_err(lit1: &Literal, lit2: &Literal) {
        match match_obj(&lit1, &lit2).primitive.as_bool() {
            boolean if boolean => panic!("\n\n lit1: {:#?}\n\n lit2: {:#?}\n", lit1, lit2),
            _ => {}
        }
    }

    #[test]
    fn ok_match_button_button() {
        let bt1 = gen_button("hola");
        let bt2 = gen_button("hola");

        match_lit_ok(&bt1, &bt2);
    }

    #[test]
    fn ok_match_array_str() {
        let bt1 = PrimitiveArray::get_literal(
            &[PrimitiveString::get_literal("hola", gen_inter())],
            gen_inter(),
        );
        let bt2 = PrimitiveString::get_literal("hola", gen_inter());

        match_lit_ok(&bt1, &bt2);
        match_lit_ok(&bt2, &bt1);
    }

    #[test]
    fn ok_match_button_str() {
        let bt1 = gen_button("hola");
        let bt2 = PrimitiveString::get_literal("hola", gen_inter());

        match_lit_ok(&bt1, &bt2);
        match_lit_ok(&bt2, &bt1);
    }

    #[test]
    fn ok_match_button_str2() {
        let bt1 = gen_button_multi_accept("hola");
        let bt2 = PrimitiveString::get_literal("toTo", gen_inter());

        match_lit_ok(&bt1, &bt2);
        match_lit_ok(&bt2, &bt1);
    }

    #[test]
    fn ok_match_button_str3() {
        let bt1 = gen_button_multi_accept("hola");
        let bt2 = PrimitiveString::get_literal("test", gen_inter());

        match_lit_ok(&bt1, &bt2);
        match_lit_ok(&bt2, &bt1);
    }

    #[test]
    fn err_match_button_button() {
        let bt1 = gen_button("hola");
        let bt2 = gen_button("nop");

        match_lit_err(&bt1, &bt2);
    }
}