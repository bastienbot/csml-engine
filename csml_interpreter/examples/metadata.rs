use csmlinterpreter::data::csml_bot::CsmlBot;
use csmlinterpreter::data::csml_flow::CsmlFlow;
use csmlinterpreter::data::event::Event;
use csmlinterpreter::data::ContextJson;
use csmlinterpreter::validate_bot;
use csmlinterpreter::{interpret, read};

const DEFAULT_ID_NAME: &str = "id";
const DEFAULT_FLOW_NAME: &str = "default";
const DEFAULT_STEP_NAME: &str = "start";
const DEFAULT_BOT_NAME: &str = "my_bot";

////////////////////////////////////////////////////////////////////////////////
// PUBLIC FUNCTION
////////////////////////////////////////////////////////////////////////////////

fn main() {
    let default_content = std::fs::read_to_string("CSML/examples/metadata.csml").unwrap();
    let default_flow = CsmlFlow::new(DEFAULT_ID_NAME, "default", &default_content, Vec::default());
    let native_component = read().unwrap();

    // Create a CsmlBot
    let bot = CsmlBot::new(
        DEFAULT_ID_NAME,
        DEFAULT_BOT_NAME,
        None,
        vec![default_flow],
        Some(native_component),
        None,
        DEFAULT_FLOW_NAME,
    );

    // Create an Event
    let event = Event::default();

    // Create a Metadata
    let metadata = serde_json::json!({
        "firstname":"Toto",
        "email":"toto@clevy.io"
    });

    // Create context
    let context = ContextJson::new(
        serde_json::json!({}),
        metadata,
        None,
        None,
        DEFAULT_STEP_NAME,
        DEFAULT_FLOW_NAME,
    );

    // Run interpreter
    let result = validate_bot(bot.to_owned());

    if result.errors.is_some() {
        dbg!(result.errors);
        return;
    }
    if result.warnings.is_some() {
        dbg!(result.warnings);
    }

    dbg!(interpret(bot, context, event, None));
}
