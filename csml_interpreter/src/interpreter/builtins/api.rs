use crate::data::error_info::ErrorInfo;
use crate::data::position::Position;
use crate::data::primitive::{PrimitiveNull, PrimitiveObject, PrimitiveString, PrimitiveType};
use crate::data::{ast::Interval, tokens::*, ApiInfo, Client, Data, Literal};
use crate::error_format::*;
use crate::interpreter::{
    builtins::{http::http_request, tools::*},
    json_to_rust::json_to_literal,
};

use std::{collections::HashMap, env};

fn format_body(
    args: &HashMap<String, Literal>,
    interval: Interval,
    client: Client,
) -> Result<Literal, ErrorInfo> {
    let mut map: HashMap<String, Literal> = HashMap::new();

    match (args.get("fn_id"), args.get(DEFAULT)) {
        (Some(literal), ..) | (.., Some(literal))
            if literal.primitive.get_type() == PrimitiveType::PrimitiveString =>
        {
            let fn_id = Literal::get_value::<String>(
                &literal.primitive,
                literal.interval,
                ERROR_FN_ID.to_owned(),
            )?;

            map.insert(
                "function_id".to_owned(),
                PrimitiveString::get_literal(&fn_id, interval),
            );
        }
        _ => {
            return Err(gen_error_info(
                Position::new(interval),
                ERROR_FN_ID.to_owned(),
            ))
        }
    };

    let sub_map = create_submap(&["fn_id", DEFAULT], &args)?;
    let client = client_to_json(&client, interval);

    map.insert(
        "data".to_owned(),
        PrimitiveObject::get_literal(&sub_map, interval),
    );
    map.insert(
        "client".to_owned(),
        PrimitiveObject::get_literal(&client, interval),
    );

    Ok(PrimitiveObject::get_literal(&map, interval))
}

fn format_headers(interval: Interval) -> HashMap<String, Literal> {
    let mut header = HashMap::new();
    header.insert(
        "content-type".to_owned(),
        PrimitiveString::get_literal("application/json", interval),
    );
    header.insert(
        "accept".to_owned(),
        PrimitiveString::get_literal("application/json,text/*", interval),
    );

    match env::var("FN_X_API_KEY") {
        Ok(value) => {
            header.insert(
                "X-Api-Key".to_owned(),
                PrimitiveString::get_literal(&value, interval),
            );
        },
        Err(_e) => {}
    };

    header
}

pub fn api(
    args: HashMap<String, Literal>,
    interval: Interval,
    data: &mut Data,
) -> Result<Literal, ErrorInfo> {
    let (client, url) = match &data.context.api_info {
        Some(ApiInfo {
            client,
            fn_endpoint,
        }) => (client.to_owned(), fn_endpoint.to_owned()),
        None => {
            return Err(gen_error_info(
                Position::new(interval),
                ERROR_FN_ENDPOINT.to_owned(),
            ))
        }
    };

    let mut http: HashMap<String, Literal> = HashMap::new();
    let header = format_headers(interval);
    let body = format_body(&args, interval, client)?;

    http.insert(
        "url".to_owned(),
        PrimitiveString::get_literal(&url, interval),
    );

    let lit_header = PrimitiveObject::get_literal(&header, interval);
    http.insert("header".to_owned(), lit_header);
    let lit_query = PrimitiveObject::get_literal(&HashMap::default(), interval);
    http.insert("query".to_owned(), lit_query);
    http.insert("body".to_owned(), body);

    if let Some(value) = http_request(&http, ureq::post, interval)?.get("data") {
        json_to_literal(value, interval)
    } else {
        Ok(PrimitiveNull::get_literal(interval))
    }
}
