use neon::prelude::*;
#[macro_use]
extern crate lazy_static;
extern crate serde_derive;
mod bets;
mod bicho;
mod enums;
mod extract;
mod scraper;
mod tests;
mod users;

fn create_bet(mut cx: FunctionContext) -> JsResult<JsString> {
    let raw_text = cx.argument::<JsString>(0)?.value(&mut cx);
    let user_id = cx.argument::<JsNumber>(1)?.value(&mut cx);
    let bet = extract::get_bet_from_text(raw_text, user_id as i64);
    match bet {
        Some(_) => Ok(cx.string("Bet created")),
        None => Ok(cx.string("Bet not valid")),
    }
}

#[tokio::main]
async fn get_once_results(mut cx: FunctionContext) -> JsResult<JsString> {
    let callback = cx.argument::<JsFunction>(0)?.root(&mut cx);
    let channel = cx.channel();

    tokio::spawn(async move {
        let results = scraper::get_once_results().await.unwrap();
        run_callback(results.clone(), callback, channel);
    })
    .await;

    Ok(cx.string(""))
}

fn run_callback(results: i32, callback: Root<JsFunction>, channel: Channel) {
    channel.send(move |mut cx| {
        let callback = callback.into_inner(&mut cx);
        let undefined = cx.undefined();
        let animals = enums::get_animal_from_number(results);
        let results = vec![
            cx.string(results.to_string()),
            cx.string(format!("{:?}", animals)),
        ];
        callback.call(&mut cx, undefined, results)?;
        Ok(())
    })
}

fn display_help(mut cx: FunctionContext) -> JsResult<JsString> {
    let help_text = extract::display_numbers_from_animals();
    Ok(cx.string(help_text))
}

fn display_animal_numbers(mut cx: FunctionContext) -> JsResult<JsString> {
    let animal = cx.argument::<JsString>(0)?.value(&mut cx);
    let help_text = extract::display_numbers_from_animal(enums::get_animal_from_text(&animal));
    Ok(cx.string(help_text))
}
#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("create_bet", create_bet)?;
    cx.export_function("display_help", display_help)?;
    cx.export_function("display_animal_numbers", display_animal_numbers)?;
    cx.export_function("get_once_results_callback", get_once_results)?;
    Ok(())
}
