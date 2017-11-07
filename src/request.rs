use std::default::Default;
use super::serde::ser::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum ParseMode {
    Text,
    Markdown,
    Html,
}
impl ParseMode {
    fn is_text(&self) -> bool {
        *self == ParseMode::Text
    }
}
impl Default for ParseMode {
    fn default() -> ParseMode {
        ParseMode::Text
    }
}
#[derive(Clone, Debug, Serialize, Default)]
pub struct Message {
    pub chat_id: i64,
    pub text: String,
    #[serde(skip_serializing_if = "ParseMode::is_text")]
    pub parse_mode: ParseMode,
    //TODO
}
impl Message {
    pub fn new(chat_id: i64, text: String) -> Message {
        Message {
            chat_id,
            text,
            ..Default::default()
        }
    }
    pub fn parse_mode(mut self, mode: ParseMode) -> Message {
        self.parse_mode = mode;
        self
    }
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct AnswerInlineQuery<InlineQueryResultType: Serialize + Default> {
    pub inline_query_id: String,
    pub results: Vec<InlineQueryResultType>,
    pub cache_time: Option<i64>,
    pub is_personal: Option<bool>,
    pub next_offset: String,
    pub switch_pm_text: Option<String>,
    pub switch_pm_parameter: Option<String>,
}
impl<InlineQueryResultType: Serialize + Default> AnswerInlineQuery<InlineQueryResultType> {
    pub fn new(
        inline_query_id: String,
        results: Vec<InlineQueryResultType>,
        next_offset: String,
    ) -> AnswerInlineQuery<InlineQueryResultType> {
        AnswerInlineQuery {
            inline_query_id,
            results,
            next_offset,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum InlineQueryResult {
    Article {
        id: String,
        title: String,
        input_message_content: InputMessageContent,
        // reply_markup: Option<InlineKeyboardMarkup>,
        // url:          Option<String>,
        // hide_url:     Option<bool>,
        description:  Option<String>,
        thumb_url:    Option<String>,
        thumb_width:  Option<u16>,
        thumb_height: Option<u16>,
    },
}

impl InlineQueryResult {

    pub fn article(
        id: String,
        title: String,
        message_text: String,
        description: String,
        // url: String,
        thumb_url: String,
        thumb_height: u16,
        thumb_width: u16) -> InlineQueryResult {

        let input_message_content = InputMessageContent{
            message_text,
            // ..Default::default()
        };

        InlineQueryResult::Article {
            id,
            title,
            input_message_content,
            description: Some(description),
            thumb_url: Some(thumb_url),
            // hide_url: None,
            // reply_markup: None,
            // url: Some(url),
            thumb_height:Some(thumb_height),
            thumb_width: Some(thumb_width),
        }
    }
}

#[derive(Clone, Debug, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub struct InputMessageContent {
    message_text:             String,
    // parse_mode:               Option<String>,
    // disable_web_page_preview: Option<bool>,
}

// #[derive(Clone, Debug, Serialize, Default)]
// #[serde(rename_all = "lowercase")]
// pub struct InlineKeyboardMarkup {
//     inline_keyboard: Vec<InlineKeyboardButton>,
// }

// #[derive(Clone, Debug, Serialize, Default)]
// #[serde(rename_all = "lowercase")]
// pub struct InlineKeyboardButton {
//     text:                             String,
//     url:                              Option<String>,
//     callback_data:                    Option<String>,
//     switch_inline_query:              Option<String>,
//     switch_inline_query_current_chat: Option<String>,
//     pay:                              Option<bool>
//     // callback_game
// }