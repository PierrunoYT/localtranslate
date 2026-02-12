use serde::{Deserialize, Serialize};

const DEFAULT_MODEL: &str = "translategemma:4b";
const SUPPORTED_MODELS: [&str; 3] = ["translategemma:4b", "translategemma:12b", "translategemma:27b"];

fn resolve_model(model: Option<String>) -> Result<String, String> {
    let requested_model = model
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(DEFAULT_MODEL);

    if SUPPORTED_MODELS.contains(&requested_model) {
        Ok(requested_model.to_string())
    } else {
        Err(format!(
            "Unsupported model '{}'. Supported models: {}",
            requested_model,
            SUPPORTED_MODELS.join(", ")
        ))
    }
}

// Language name mapping for the prompt template (TranslateGemma supported languages)
fn get_language_name(code: &str) -> &str {
    match code {
        "en" => "English",
        "es" => "Spanish",
        "fr" => "French",
        "de" => "German",
        "it" => "Italian",
        "pt" => "Portuguese",
        "ja" => "Japanese",
        "zh" | "zh-Hans" => "Chinese",
        "zh-Hant" => "Chinese",
        "ar" => "Arabic",
        "ru" => "Russian",
        "ko" => "Korean",
        "hi" => "Hindi",
        "nl" => "Dutch",
        "aa" => "Afar",
        "ab" => "Abkhazian",
        "af" => "Afrikaans",
        "ak" => "Akan",
        "am" => "Amharic",
        "an" => "Aragonese",
        "as" => "Assamese",
        "az" => "Azerbaijani",
        "ba" => "Bashkir",
        "be" => "Belarusian",
        "bg" => "Bulgarian",
        "bm" => "Bambara",
        "bn" => "Bengali",
        "bo" => "Tibetan",
        "br" => "Breton",
        "bs" => "Bosnian",
        "ca" => "Catalan",
        "ce" => "Chechen",
        "co" => "Corsican",
        "cs" => "Czech",
        "cv" => "Chuvash",
        "cy" => "Welsh",
        "da" => "Danish",
        "dv" => "Divehi",
        "dz" => "Dzongkha",
        "ee" => "Ewe",
        "el" => "Greek",
        "eo" => "Esperanto",
        "et" => "Estonian",
        "eu" => "Basque",
        "fa" => "Persian",
        "ff" => "Fulah",
        "fi" => "Finnish",
        "fo" => "Faroese",
        "fy" => "Western Frisian",
        "ga" => "Irish",
        "gd" => "Scottish Gaelic",
        "gl" => "Galician",
        "gn" => "Guarani",
        "gu" => "Gujarati",
        "ha" => "Hausa",
        "he" => "Hebrew",
        "hr" => "Croatian",
        "ht" => "Haitian",
        "hu" => "Hungarian",
        "hy" => "Armenian",
        "id" => "Indonesian",
        "ig" => "Igbo",
        "is" => "Icelandic",
        "jv" => "Javanese",
        "ka" => "Georgian",
        "ki" => "Kikuyu",
        "kk" => "Kazakh",
        "kl" => "Kalaallisut",
        "km" => "Khmer",
        "kn" => "Kannada",
        "ks" => "Kashmiri",
        "ku" => "Kurdish",
        "kw" => "Cornish",
        "ky" => "Kyrgyz",
        "la" => "Latin",
        "lb" => "Luxembourgish",
        "lg" => "Ganda",
        "ln" => "Lingala",
        "lo" => "Lao",
        "lt" => "Lithuanian",
        "lu" => "Luba-Katanga",
        "lv" => "Latvian",
        "mg" => "Malagasy",
        "mi" => "Maori",
        "mk" => "Macedonian",
        "ml" => "Malayalam",
        "mn" => "Mongolian",
        "mr" => "Marathi",
        "ms" => "Malay",
        "mt" => "Maltese",
        "my" => "Burmese",
        "nb" => "Norwegian Bokmål",
        "nd" => "North Ndebele",
        "ne" => "Nepali",
        "nn" => "Norwegian Nynorsk",
        "no" => "Norwegian",
        "nr" => "South Ndebele",
        "nv" => "Navajo",
        "ny" => "Chichewa",
        "oc" => "Occitan",
        "om" => "Oromo",
        "or" => "Oriya",
        "os" => "Ossetian",
        "pa" => "Punjabi",
        "pl" => "Polish",
        "ps" => "Pashto",
        "qu" => "Quechua",
        "rm" => "Romansh",
        "rn" => "Rundi",
        "ro" => "Romanian",
        "rw" => "Kinyarwanda",
        "sa" => "Sanskrit",
        "sc" => "Sardinian",
        "sd" => "Sindhi",
        "se" => "Northern Sami",
        "sg" => "Sango",
        "si" => "Sinhala",
        "sk" => "Slovak",
        "sl" => "Slovenian",
        "sn" => "Shona",
        "so" => "Somali",
        "sq" => "Albanian",
        "sr" => "Serbian",
        "ss" => "Swati",
        "st" => "Southern Sotho",
        "su" => "Sundanese",
        "sv" => "Swedish",
        "sw" => "Swahili",
        "ta" => "Tamil",
        "te" => "Telugu",
        "tg" => "Tajik",
        "th" => "Thai",
        "ti" => "Tigrinya",
        "tk" => "Turkmen",
        "tl" => "Tagalog",
        "tn" => "Tswana",
        "to" => "Tonga",
        "tr" => "Turkish",
        "ts" => "Tsonga",
        "tt" => "Tatar",
        "ug" => "Uyghur",
        "uk" => "Ukrainian",
        "ur" => "Urdu",
        "uz" => "Uzbek",
        "ve" => "Venda",
        "vi" => "Vietnamese",
        "vo" => "Volapük",
        "wa" => "Walloon",
        "wo" => "Wolof",
        "xh" => "Xhosa",
        "yi" => "Yiddish",
        "yo" => "Yoruba",
        "za" => "Zhuang",
        "zu" => "Zulu",
        _ => code,
    }
}

// Build the TranslateGemma prompt template
fn build_translation_prompt(source_lang: &str, target_lang: &str, text: &str) -> String {
    let source_name = get_language_name(source_lang);
    let target_name = get_language_name(target_lang);
    
    format!(
        "You are a professional {} ({}) to {} ({}) translator. Your goal is to accurately convey the meaning and nuances of the original {} text while adhering to {} grammar, vocabulary, and cultural sensitivities.\n\nProduce only the {} translation, without any additional explanations or commentary. Please translate the following {} text into {}:\n\n\n{}",
        source_name, source_lang, target_name, target_lang, source_name, target_name, target_name, source_name, target_name, text
    )
}

#[derive(Serialize, Deserialize, Debug)]
struct OllamaMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct OllamaRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct OllamaResponse {
    message: OllamaMessage,
}

#[tauri::command]
async fn translate_text(
    source_lang: String,
    target_lang: String,
    text: String,
    model: Option<String>,
) -> Result<String, String> {
    if text.trim().is_empty() {
        return Err("Text cannot be empty".to_string());
    }

    let selected_model = resolve_model(model)?;

    // Build the prompt using TranslateGemma's required format
    let prompt = build_translation_prompt(&source_lang, &target_lang, &text);

    // Create the request to Ollama API
    let request_body = OllamaRequest {
        model: selected_model.clone(),
        messages: vec![OllamaMessage {
            role: "user".to_string(),
            content: prompt,
        }],
        stream: false,
    };

    // Send request to Ollama
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:11434/api/chat")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| {
            format!(
                "Failed to connect to Ollama. Please ensure Ollama is running and '{}' is installed.\nError: {}",
                selected_model, e
            )
        })?;

    if !response.status().is_success() {
        return Err(format!(
            "Ollama API returned error: {}. Make sure '{}' model is installed.",
            response.status(),
            selected_model
        ));
    }

    let ollama_response: OllamaResponse = response.json().await.map_err(|e| {
        format!("Failed to parse Ollama response: {}", e)
    })?;

    Ok(ollama_response.message.content)
}

#[derive(Serialize, Deserialize, Debug)]
struct OllamaTagsResponse {
    models: Vec<OllamaModel>,
}

#[derive(Serialize, Deserialize, Debug)]
struct OllamaModel {
    name: String,
}

#[tauri::command]
async fn check_ollama_status(model: Option<String>) -> Result<String, String> {
    let selected_model = resolve_model(model)?;
    let client = reqwest::Client::new();
    
    // Check if Ollama is running
    let response = client
        .get("http://localhost:11434/api/tags")
        .send()
        .await
        .map_err(|_| "Ollama is not running. Please start Ollama with: ollama serve".to_string())?;

    // Check if the model is installed
    let tags: OllamaTagsResponse = response
        .json()
        .await
        .map_err(|_| "Failed to parse Ollama response".to_string())?;

    let model_installed = tags
        .models
        .iter()
        .any(|m| m.name.starts_with(&selected_model));

    if model_installed {
        Ok(format!("{} is ready", selected_model))
    } else {
        Err(format!(
            "{} model not found. Please install it with:\n\nollama run {}",
            selected_model, selected_model
        ))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![translate_text, check_ollama_status])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
