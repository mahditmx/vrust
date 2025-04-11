use crate::ccolor;
use regex::Regex;

pub fn render(text:String) -> String {
    let secret_char = '\u{7F}';
    let regex = Regex::new(r"(#+)( ?.+)").unwrap(); // net to fix


    let result = regex.replace_all(text.as_str(), |caps: &regex::Captures| {
        let group1 = format!("{}{}{}",ccolor::colors::BOLD_RED,&caps[1], ccolor::colors::RESET);
        let group2 = format!("{}{}{}",ccolor::colors::BOLD_MAGENTA ,&caps[2], ccolor::colors::RESET);
        format!("{}{}", group1, group2)
    });


    use regex::Regex;

    let regex = Regex::new(r"((\*\*)([^*]+)(\*\*))").unwrap();
    let result = regex.replace_all(&result, |caps: &regex::Captures| {
        // let group1 = format!("{}{}{}",ccolor::colors::BOLD,&caps[1], ccolor::colors::RESET);
        // let group2 = format!("{}{}{}",ccolor::colors::BOLD ,&caps[2], ccolor::colors::RESET);



        format!("{}{}{}{}{}",secret_char,ccolor::colors::BOLD,&caps[3],ccolor::colors::RESET,secret_char)


    });




    let regex = Regex::new(r"(\*)([^*><]+)(\*)").unwrap();
    let result = regex.replace_all(&result, |caps: &regex::Captures| {



        let group0 = format!("{}{}{}{}{}{}{}{}",ccolor::colors::CYAN,&caps[1],ccolor::colors::RESET, ccolor::colors::ITALIC,&caps[2], ccolor::colors::BLUE, &caps[3],ccolor::colors::RESET);


        group0
    });

    // back to org char
    let result = result.replace(secret_char, format!("{}**{}",ccolor::colors::CYAN,ccolor::colors::RESET).as_str());



    let regex = Regex::new(r"(`)([^*><]+)(`)").unwrap();
    let result = regex.replace_all(&result, |caps: &regex::Captures| {



        let group0 = format!("{}{}{}{}{}{}{}{}",ccolor::colors::CYAN,&caps[1],ccolor::colors::RESET, ccolor::colors::RED,&caps[2], ccolor::colors::BLUE, &caps[3],ccolor::colors::RESET);


        group0
    });
    let regex = Regex::new(r"^(>)( ?.+)").unwrap();
    let result = regex.replace_all(&result, |caps: &regex::Captures| {



        let group0 = format!("{}{}{}{}{}{}",ccolor::colors::BOLD_RED ,&caps[1],ccolor::colors::RESET,ccolor::colors::BLUE, &caps[2],ccolor::colors::RESET);


        group0
    });

    let result = result.replace("[x]",format!("{}[x]{}",ccolor::colors::GREEN,ccolor::colors::RESET).as_str());


    result.to_string()


}
