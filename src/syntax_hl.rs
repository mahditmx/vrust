use std::fmt::format;
use crate::{ccolor, Highlight};
use regex::Regex;
use ccolor::colors;

pub fn render(text:String, highlight: Highlight) -> String {


    if highlight.language == "markdown".to_string(){
        render_mark_down(text.clone())
    }
    else if highlight.language == "text".to_string(){
        text
    }
    else if highlight.language == "rust".to_string(){
        let text = render_common_program_language(text,0);
        render_rust(text,0)
    }
    else{
        render_common_program_language(text,0)
    }


}


fn render_mark_down(text:String) -> String {
    let secret_char = '\u{7F}';
    let regex = Regex::new(r"(#+)( ?.+)").unwrap(); // net to fix


    let result = regex.replace_all(text.as_str(), |caps: &regex::Captures| {
        let group1 = format!("{}{}{}",colors::BOLD_RED,&caps[1], colors::RESET);
        let group2 = format!("{}{}{}",colors::BOLD_MAGENTA ,&caps[2], colors::RESET);
        format!("{}{}", group1, group2)
    });



    let regex = Regex::new(r"((\*\*)([^*]+)(\*\*))").unwrap();
    let result = regex.replace_all(&result, |caps: &regex::Captures| {
        // let group1 = format!("{}{}{}",colors::BOLD,&caps[1], colors::RESET);
        // let group2 = format!("{}{}{}",colors::BOLD ,&caps[2], colors::RESET);



        format!("{}{}{}{}{}",secret_char,colors::BOLD,&caps[3],colors::RESET,secret_char)


    });




    let regex = Regex::new(r"(\*)([^*><]+)(\*)").unwrap();
    let result = regex.replace_all(&result, |caps: &regex::Captures| {



        let group0 = format!("{}{}{}{}{}{}{}{}",colors::CYAN,&caps[1],colors::RESET, colors::ITALIC,&caps[2], colors::BLUE, &caps[3],colors::RESET);


        group0
    });

    // back to org char
    let result = result.replace(secret_char, format!("{}**{}",colors::CYAN,colors::RESET).as_str());



    let regex = Regex::new(r"(`)([^*><]+)(`)").unwrap();
    let result = regex.replace_all(&result, |caps: &regex::Captures| {



        let group0 = format!("{}{}{}{}{}{}{}{}",colors::CYAN,&caps[1],colors::RESET, colors::RED,&caps[2], colors::BLUE, &caps[3],colors::RESET);


        group0
    });
    let regex = Regex::new(r"^(>)( ?.+)").unwrap();
    let result = regex.replace_all(&result, |caps: &regex::Captures| {



        let group0 = format!("{}{}{}{}{}{}",colors::BOLD_RED ,&caps[1],colors::RESET,colors::BLUE, &caps[2],colors::RESET);


        group0
    });

    let result = result.replace("[x]",format!("{}[x]{}",colors::GREEN,colors::RESET).as_str());


    result.to_string()

}


fn render_common_program_language(text:String, highlight: usize) -> String {
    // let secret_char = '\u{7F}';

    let regex = Regex::new(r#""([^"]+)""#).unwrap();
    let result = regex.replace_all(&text, |caps: &regex::Captures| {


        format!("{}\"{}\"{}", colors::RED, &caps[1], colors::RESET)
    });




    let regex = Regex::new(r#"(?P<dot>\.?)\b(?P<name>[a-zA-Z_][a-zA-Z0-9_]*\s*)\((?P<args>[^()]*)\)"#).unwrap();
    let result = regex.replace_all(&result, |caps: &regex::Captures| {


        format!("{}{}{}{}{}({}{}{}){}",&caps["dot"],colors::YELLOW,&caps["name"],colors::RESET,colors::BLUE,colors::RESET,&caps["args"],colors::BLUE,colors::RESET)
    });




    let result = result
        .replace("{",format!("{}{}{}",colors::CYAN,"{",colors::RESET).as_str())
        .replace("}",format!("{}{}{}",colors::CYAN,"}",colors::RESET).as_str());

    result.to_string()

}
fn render_rust(text:String, lvl:u8) -> String {


    let regex = Regex::new(r"(?m)((?:[a-zA-Z_][a-zA-Z0-9_]*::)+)([a-zA-Z_][a-zA-Z0-9_]*(?:<[^>]+>)?(?:\(\))?)").unwrap();

    let result = regex.replace_all(&text, |caps: &regex::Captures| {

        let body = format!("{}{}",&caps[1],&caps[2]).replace("::",format!("{}::{}",colors::RESET,colors::GREEN).as_str());
        format!("{}{}{}",colors::GREEN,body,colors::RESET)
    });


    let regex = Regex::new(r"([a-zA-Z0-9_]+)(\.)([a-zA-Z0-9_]+)").unwrap();
    let result = regex.replace_all(&result, |caps: &regex::Captures| {


        format!("{}{}{}{}{}",&caps[1],&caps[2],colors::BLUE,&caps[3],colors::RESET)
    });

    let regex = Regex::new(r"(\b\d+\b)").unwrap();
    let result = regex.replace_all(&result, |caps: &regex::Captures| {


        format!("{}{}{}",colors::BLUE,&caps[1],colors::RESET)
    });


    let regex = Regex::new(r"(//.*)").unwrap();
    let result = regex.replace_all(&result, |caps: &regex::Captures| {


        format!("{}{}{}{}",colors::ITALIC,colors::CYAN,&caps[1],colors::RESET)
    });

    let mut result = result.to_string();

    result = Regex::new(r"\btrue\b").unwrap()
        .replace_all(&result, format!("{}true{}", colors::MAGENTA, colors::RESET).as_str())
        .to_string();

    result = Regex::new(r"\bfalse\b").unwrap()
        .replace_all(&result, format!("{}false{}", colors::MAGENTA, colors::RESET).as_str())
        .to_string();

    result = Regex::new(r"\bnone\b").unwrap()
        .replace_all(&result, format!("{}none{}", colors::MAGENTA, colors::RESET).as_str())
        .to_string();

    result = Regex::new(r"\bnull\b").unwrap()
        .replace_all(&result, format!("{}null{}", colors::MAGENTA, colors::RESET).as_str())
        .to_string();

    result = Regex::new(r"\bfor\b").unwrap()
        .replace_all(&result, format!("{}for{}", colors::MAGENTA, colors::RESET).as_str())
        .to_string();

    result = Regex::new(r"\bin\b").unwrap()
        .replace_all(&result, format!("{}in{}", colors::MAGENTA, colors::RESET).as_str())
        .to_string();

    result = Regex::new(r"\bif\b").unwrap()
        .replace_all(&result, format!("{}if{}", colors::MAGENTA, colors::RESET).as_str())
        .to_string();

    result = Regex::new(r"\belse\b").unwrap()
        .replace_all(&result, format!("{}else{}", colors::MAGENTA, colors::RESET).as_str())
        .to_string();

    result = Regex::new(r"\bSome\b").unwrap()
        .replace_all(&result, format!("{}Some{}", colors::MAGENTA, colors::RESET).as_str())
        .to_string();

    result = Regex::new(r"\bOk\b").unwrap()
        .replace_all(&result, format!("{}Ok{}", colors::MAGENTA, colors::RESET).as_str())
        .to_string();

    result = Regex::new(r"\blet\b").unwrap()
        .replace_all(&result, format!("{}let{}", colors::BOLD_YELLOW, colors::RESET).as_str())
        .to_string();

    result = Regex::new(r"\bmut\b").unwrap()
        .replace_all(&result, format!("{}mut{}", colors::YELLOW, colors::RESET).as_str())
        .to_string();

    result = Regex::new(r"\bcontinue\b").unwrap()
        .replace_all(&result, format!("{}continue{}", colors::YELLOW, colors::RESET).as_str())
        .to_string();

    result = Regex::new(r"\bbreak\b").unwrap()
        .replace_all(&result, format!("{}continue{}", colors::YELLOW, colors::RESET).as_str())
        .to_string();

    result = Regex::new(r"\bas\b").unwrap()
        .replace_all(&result, format!("{}as{}", colors::YELLOW, colors::RESET).as_str())
        .to_string();

    result = Regex::new(r"\bu16\b").unwrap()
        .replace_all(&result, format!("{}u16{}", colors::YELLOW, colors::RESET).as_str())
        .to_string();

    result = Regex::new(r"\busize\b").unwrap()
        .replace_all(&result, format!("{}usize{}", colors::YELLOW, colors::RESET).as_str())
        .to_string();

    result = Regex::new(r"\bu8\b").unwrap()
        .replace_all(&result, format!("{}u8{}", colors::YELLOW, colors::RESET).as_str())
        .to_string();

    result = Regex::new(r"\bu32\b").unwrap()
        .replace_all(&result, format!("{}u32{}", colors::YELLOW, colors::RESET).as_str())
        .to_string();

    result = Regex::new(r"\buse\b").unwrap()
        .replace_all(&result, format!("{}use{}", colors::YELLOW, colors::RESET).as_str())
        .to_string();

    result = Regex::new(r"\bpub\b").unwrap()
        .replace_all(&result, format!("{}pub{}", colors::YELLOW, colors::RESET).as_str())
        .to_string();

    result = Regex::new(r"\bmod\b").unwrap()
        .replace_all(&result, format!("{}u32{}", colors::YELLOW, colors::RESET).as_str())
        .to_string();


    result


}














