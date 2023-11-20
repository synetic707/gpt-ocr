#[macro_use] extern crate log;

use clap::Parser;
use log::LevelFilter;
use arboard::Clipboard;
use simple_logger::SimpleLogger;

use crate::requests::{OpenAIRequest, Message, Content, ImageUrl};


#[path = "api/openai.rs"]
mod openai;

#[path = "api/requests/openairequest.rs"]
mod requests;

#[path = "api/responses/openairesponse.rs"]
mod responses;


/// A simple CLI tool to read text from images (OCR)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File path to an image
    #[arg(short, long, env = "OCR_INPUT")]
    input: String,

    /// OpenAI API Key used for GPT calls
    #[arg(short, long, env = "OCR_APIKEY")]
    apikey: String,
}

fn main() {
    SimpleLogger::new().with_level(LevelFilter::Info).with_module_level("gpt-ocr", LevelFilter::Debug).init().unwrap();
    let args = Args::parse();

    let request = OpenAIRequest {
        model: "gpt-4-vision-preview".to_string(),
        messages: vec![
            Message {
                role: "user".to_string(),
                content: vec![
                    Content::Text { text: "You are now an OCR Tool. Respond what is written in the picture as is. Do not interpret or describe. Do not add any code markup. Do not include ```".to_string() },
                    Content::ImageUrl {
                        image_url: ImageUrl {
                            url: format!("data:image/jpeg;base64,{}", openai::encode_image(&args.input)),
                        },
                    },
                ],
            },
        ],
        max_tokens: 300,
    };
    
    
    let response = openai::get_gpt4_response(request, args.apikey);
    println!("{}", response);

    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(response).unwrap()
}
