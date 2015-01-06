use std::collections::TreeMap;
use serialize::json;
use serialize::json::{ToJson, Json};
use regex::Captures;

use common::{WebDriverResult, WebDriverError, ErrorStatus, Nullable, WebElement, FrameId, LocatorStrategy};
use response::Date; //TODO: Put all these types in a specific file
use messagebuilder::MatchType;


#[deriving(PartialEq)]
pub enum WebDriverCommand {
    NewSession,
    DeleteSession,
    Get(GetParameters),
    GetCurrentUrl,
    GoBack,
    GoForward,
    Refresh,
    GetTitle,
    GetWindowHandle,
    GetWindowHandles,
    Close,
    SetWindowSize(WindowSizeParameters),
    GetWindowSize,
    MaximizeWindow,
//    FullscreenWindow // Not supported in marionette
    SwitchToWindow(SwitchToWindowParameters),
    SwitchToFrame(SwitchToFrameParameters),
    SwitchToParentFrame,
    FindElement(LocatorParameters),
    FindElements(LocatorParameters),
    IsDisplayed(WebElement),
    IsSelected(WebElement),
    GetElementAttribute(WebElement, String),
    GetCSSValue(WebElement, String),
    GetElementText(WebElement),
    GetElementTagName(WebElement),
    GetElementRect(WebElement),
    IsEnabled(WebElement),
    ExecuteScript(JavascriptCommandParameters),
    ExecuteAsyncScript(JavascriptCommandParameters),
    GetCookie(GetCookieParameters),
    AddCookie(AddCookieParameters),
    SetTimeouts(TimeoutsParameters),
    //Actions(ActionsParameters)
    DismissAlert,
    AcceptAlert,
    GetAlertText,
    SendAlertText(SendAlertTextParameters),
    TakeScreenshot(TakeScreenshotParameters)
}

#[deriving(PartialEq)]
pub struct WebDriverMessage {
    pub session_id: Option<String>,
    pub command: WebDriverCommand
}

impl WebDriverMessage {
    pub fn new(session_id: Option<String>, command: WebDriverCommand) -> WebDriverMessage {
        WebDriverMessage {
            session_id: session_id,
            command: command
        }
    }

    pub fn from_http(match_type: MatchType, params: &Captures, body: &str) -> WebDriverResult<WebDriverMessage> {
        let session_id = WebDriverMessage::get_session_id(params);
        let body_data = if body != "" {
            debug!("Got request body {}", body);
            match json::from_str(body) {
                Ok(x) => x,
                Err(_) => return Err(WebDriverError::new(ErrorStatus::UnknownError,
                                                         format!("Failed to decode request body as json: {}", body).as_slice()))
            }
        } else {
            json::Null
        };
        let command = match match_type {
            MatchType::NewSession => WebDriverCommand::NewSession,
            MatchType::DeleteSession => WebDriverCommand::DeleteSession,
            MatchType::Get => {
                let parameters: GetParameters = try!(Parameters::from_json(&body_data));
                WebDriverCommand::Get(parameters)
            },
            MatchType::GetCurrentUrl => WebDriverCommand::GetCurrentUrl,
            MatchType::GoBack => WebDriverCommand::GoBack,
            MatchType::GoForward => WebDriverCommand::GoForward,
            MatchType::Refresh => WebDriverCommand::Refresh,
            MatchType::GetTitle => WebDriverCommand::GetTitle,
            MatchType::GetWindowHandle => WebDriverCommand::GetWindowHandle,
            MatchType::GetWindowHandles => WebDriverCommand::GetWindowHandles,
            MatchType::Close => WebDriverCommand::Close,
            MatchType::SetTimeouts => {
                let parameters: TimeoutsParameters = try!(Parameters::from_json(&body_data));
                WebDriverCommand::SetTimeouts(parameters)
            },
            MatchType::SetWindowSize => {
                let parameters: WindowSizeParameters = try!(Parameters::from_json(&body_data));
                WebDriverCommand::SetWindowSize(parameters)
            },
            MatchType::GetWindowSize => WebDriverCommand::GetWindowSize,
            MatchType::MaximizeWindow => WebDriverCommand::MaximizeWindow,
            MatchType::SwitchToWindow => {
                let parameters: SwitchToWindowParameters = try!(Parameters::from_json(&body_data));
                WebDriverCommand::SwitchToWindow(parameters)
            }
            MatchType::SwitchToFrame => {
                let parameters: SwitchToFrameParameters = try!(Parameters::from_json(&body_data));
                WebDriverCommand::SwitchToFrame(parameters)
            },
            MatchType::SwitchToParentFrame => WebDriverCommand::SwitchToParentFrame,
            MatchType::FindElement => {
                let parameters: LocatorParameters = try!(Parameters::from_json(&body_data));
                WebDriverCommand::FindElement(parameters)
            },
            MatchType::FindElements => {
                let parameters: LocatorParameters = try!(Parameters::from_json(&body_data));
                WebDriverCommand::FindElements(parameters)
            },
            MatchType::IsDisplayed => {
                let element = WebElement::new(params.name("elementId").to_string());
                WebDriverCommand::IsDisplayed(element)
            },
            MatchType::IsSelected => {
                let element = WebElement::new(params.name("elementId").to_string());
                WebDriverCommand::IsSelected(element)
            },
            MatchType::GetElementAttribute => {
                let element = WebElement::new(params.name("elementId").to_string());
                let attr = params.name("name").to_string();
                WebDriverCommand::GetElementAttribute(element, attr)
            },
            MatchType::GetCSSValue => {
                let element = WebElement::new(params.name("elementId").to_string());
                let property = params.name("propertyName").to_string();
                WebDriverCommand::GetCSSValue(element, property)
            },
            MatchType::GetElementText => {
                let element = WebElement::new(params.name("elementId").to_string());
                WebDriverCommand::GetElementText(element)
            },
            MatchType::GetElementTagName => {
                let element = WebElement::new(params.name("elementId").to_string());
                WebDriverCommand::GetElementTagName(element)
            },
            MatchType::GetElementRect => {
                let element = WebElement::new(params.name("elementId").to_string());
                WebDriverCommand::GetElementRect(element)
            },
            MatchType::IsEnabled => {
                let element = WebElement::new(params.name("elementId").to_string());
                WebDriverCommand::IsEnabled(element)
            },
            MatchType::ExecuteScript => {
                let parameters: JavascriptCommandParameters = try!(Parameters::from_json(&body_data));
                WebDriverCommand::ExecuteScript(parameters)
            },
            MatchType::ExecuteAsyncScript => {
                let parameters: JavascriptCommandParameters = try!(Parameters::from_json(&body_data));
                WebDriverCommand::ExecuteAsyncScript(parameters)
            },
            MatchType::GetCookie => {
                let parameters: GetCookieParameters = try!(Parameters::from_json(&body_data));
                WebDriverCommand::GetCookie(parameters)
            },
            MatchType::AddCookie => {
                let parameters: AddCookieParameters = try!(Parameters::from_json(&body_data));
                WebDriverCommand::AddCookie(parameters)
            },
            MatchType::DismissAlert => {
                WebDriverCommand::DismissAlert
            },
            MatchType::AcceptAlert => {
                WebDriverCommand::AcceptAlert
            },
            MatchType::GetAlertText => {
                WebDriverCommand::GetAlertText
            },
            MatchType::SendAlertText => {
                let parameters: SendAlertTextParameters = try!(Parameters::from_json(&body_data));
                WebDriverCommand::SendAlertText(parameters)
            }
            MatchType::TakeScreenshot => {
                let parameters: TakeScreenshotParameters = try!(Parameters::from_json(&body_data));
                WebDriverCommand::TakeScreenshot(parameters)
            }
        };
        Ok(WebDriverMessage::new(session_id, command))
    }

    fn get_session_id(params: &Captures) -> Option<String> {
        match params.name("sessionId") {
            "" => None,
            x => Some(x.to_string())
        }
    }
}

impl ToJson for WebDriverMessage {
    fn to_json(&self) -> json::Json {
        let mut data = TreeMap::new();
        let parameters = match self.command {
            WebDriverCommand::NewSession |
            WebDriverCommand::DeleteSession | WebDriverCommand::GetCurrentUrl |
            WebDriverCommand::GoBack | WebDriverCommand::GoForward | WebDriverCommand::Refresh |
            WebDriverCommand::GetTitle | WebDriverCommand::GetWindowHandle |
            WebDriverCommand::GetWindowHandles | WebDriverCommand::Close |
            WebDriverCommand::GetWindowSize | WebDriverCommand::MaximizeWindow |
            WebDriverCommand::SwitchToParentFrame | WebDriverCommand::IsDisplayed(_) |
            WebDriverCommand::IsSelected(_) | WebDriverCommand::GetElementAttribute(_, _) |
            WebDriverCommand::GetCSSValue(_, _) | WebDriverCommand::GetElementText(_) |
            WebDriverCommand::GetElementTagName(_) | WebDriverCommand::GetElementRect(_) |
            WebDriverCommand::IsEnabled(_) | WebDriverCommand::AddCookie(_) |
            WebDriverCommand::DismissAlert | WebDriverCommand::AcceptAlert |
            WebDriverCommand::GetAlertText => {
                None
            },
            WebDriverCommand::Get(ref x) => Some(x.to_json()),
            WebDriverCommand::SetTimeouts(ref x) => Some(x.to_json()),
            WebDriverCommand::SetWindowSize(ref x) => Some(x.to_json()),
            WebDriverCommand::SwitchToWindow(ref x) => Some(x.to_json()),
            WebDriverCommand::SwitchToFrame(ref x) => Some(x.to_json()),
            WebDriverCommand::FindElement(ref x) => Some(x.to_json()),
            WebDriverCommand::FindElements(ref x) => Some(x.to_json()),
            WebDriverCommand::ExecuteScript(ref x) |
            WebDriverCommand::ExecuteAsyncScript(ref x) => Some(x.to_json()),
            WebDriverCommand::GetCookie(ref x) => Some(x.to_json()),
            WebDriverCommand::SendAlertText(ref x) => Some(x.to_json()),
            WebDriverCommand::TakeScreenshot(ref x) => Some(x.to_json())
        };
        if parameters.is_some() {
            data.insert("parameters".to_string(), parameters.unwrap());
        }
        json::Object(data)
    }
}

trait Parameters {
    fn from_json(body: &json::Json) -> WebDriverResult<Self>;
}

#[deriving(PartialEq)]
pub struct GetParameters {
    url: String
}

impl Parameters for GetParameters {
    fn from_json(body: &json::Json) -> WebDriverResult<GetParameters> {
        let data = try_opt!(body.as_object(), ErrorStatus::UnknownError,
                            "Message body was not an object");
        let url = try_opt!(
            try_opt!(data.get("url"),
                     ErrorStatus::InvalidArgument,
                     "Missing 'url' parameter").as_string(),
            ErrorStatus::InvalidArgument,
            "'url' not a string");
        return Ok(GetParameters {
            url: url.to_string()
        })
    }
}

impl ToJson for GetParameters {
    fn to_json(&self) -> json::Json {
        let mut data = TreeMap::new();
        data.insert("url".to_string(), self.url.to_json());
        json::Object(data)
    }
}

#[deriving(PartialEq)]
pub struct TimeoutsParameters {
    type_: String,
    ms: u64
}

impl Parameters for TimeoutsParameters {
    fn from_json(body: &json::Json) -> WebDriverResult<TimeoutsParameters> {
        let data = try_opt!(body.as_object(), ErrorStatus::UnknownError,
                            "Message body was not an object");
        let type_ = try_opt!(
            try_opt!(data.get("type"),
                     ErrorStatus::InvalidArgument,
                     "Missing 'type' parameter").as_string(),
            ErrorStatus::InvalidArgument,
            "'type' not a string");

        let ms = try_opt!(
            try_opt!(data.get("ms"),
                     ErrorStatus::InvalidArgument,
                     "Missing 'ms' parameter").as_u64(),
            ErrorStatus::InvalidArgument,
            "'ms' not an integer");
        return Ok(TimeoutsParameters {
            type_: type_.to_string(),
            ms: ms
        })
    }
}

impl ToJson for TimeoutsParameters {
    fn to_json(&self) -> json::Json {
        let mut data = TreeMap::new();
        data.insert("type".to_string(), self.type_.to_json());
        data.insert("ms".to_string(), self.ms.to_json());
        json::Object(data)
    }
}

#[deriving(PartialEq)]
pub struct WindowSizeParameters {
    width: u64,
    height: u64
}

impl Parameters for WindowSizeParameters {
    fn from_json(body: &json::Json) -> WebDriverResult<WindowSizeParameters> {
        let data = try_opt!(body.as_object(), ErrorStatus::UnknownError,
                            "Message body was not an object");
        let height = try_opt!(
            try_opt!(data.get("height"),
                     ErrorStatus::InvalidArgument,
                     "Missing 'height' parameter").as_u64(),
            ErrorStatus::InvalidArgument,
            "'height' is not a positive integer");
        let width = try_opt!(
            try_opt!(data.get("width"),
                     ErrorStatus::InvalidArgument,
                     "Missing width parameter").as_u64(),
            ErrorStatus::InvalidArgument,
            "'width' is not a positive integer");
        return Ok(WindowSizeParameters {
            height: height,
            width: width
        })
    }
}

impl ToJson for WindowSizeParameters {
    fn to_json(&self) -> json::Json {
        let mut data = TreeMap::new();
        data.insert("width".to_string(), self.width.to_json());
        data.insert("height".to_string(), self.height.to_json());
        json::Object(data)
    }
}

#[deriving(PartialEq)]
pub struct SwitchToWindowParameters {
    handle: String
}

impl Parameters for SwitchToWindowParameters {
    fn from_json(body: &json::Json) -> WebDriverResult<SwitchToWindowParameters> {
        let data = try_opt!(body.as_object(), ErrorStatus::UnknownError,
                            "Message body was not an object");
        let handle = try_opt!(
            try_opt!(data.get("handle"),
                     ErrorStatus::InvalidArgument,
                     "Missing 'handle' parameter").as_string(),
            ErrorStatus::InvalidArgument,
            "'handle' not a string");
        return Ok(SwitchToWindowParameters {
            handle: handle.to_string()
        })
    }
}

impl ToJson for SwitchToWindowParameters {
    fn to_json(&self) -> json::Json {
        let mut data = TreeMap::new();
        data.insert("handle".to_string(), self.handle.to_json());
        json::Object(data)
    }
}

#[deriving(PartialEq)]
pub struct LocatorParameters {
    using: LocatorStrategy,
    value: String
}

impl Parameters for LocatorParameters {
    fn from_json(body: &json::Json) -> WebDriverResult<LocatorParameters> {
        let data = try_opt!(body.as_object(), ErrorStatus::UnknownError,
                            "Message body was not an object");

        let using = try!(LocatorStrategy::from_json(
            try_opt!(data.get("using"),
                     ErrorStatus::InvalidArgument,
                     "Missing 'using' parameter")));

        let value = try_opt!(
            try_opt!(data.get("value"),
                     ErrorStatus::InvalidArgument,
                     "Missing 'using' parameter").as_string(),
            ErrorStatus::InvalidArgument,
            "Could not convert using to string").into_string();

        return Ok(LocatorParameters {
            using: using,
            value: value
        })
    }
}

impl ToJson for LocatorParameters {
    fn to_json(&self) -> json::Json {
        let mut data = TreeMap::new();
        data.insert("using".to_string(), self.using.to_json());
        data.insert("value".to_string(), self.value.to_json());
        json::Object(data)
    }
}

#[deriving(PartialEq)]
pub struct SwitchToFrameParameters {
    pub id: FrameId
}

impl Parameters for SwitchToFrameParameters {
    fn from_json(body: &json::Json) -> WebDriverResult<SwitchToFrameParameters> {
        let data = try_opt!(body.as_object(),
                            ErrorStatus::UnknownError,
                            "Message body was not an object");
        let id = try!(FrameId::from_json(try_opt!(data.get("id"),
                                                  ErrorStatus::UnknownError,
                                                  "Missing 'id' parameter")));

        Ok(SwitchToFrameParameters {
            id: id
        })
    }
}

impl ToJson for SwitchToFrameParameters {
    fn to_json(&self) -> json::Json {
        let mut data = TreeMap::new();
        data.insert("id".to_string(), self.id.to_json());
        json::Object(data)
    }
}

#[deriving(PartialEq)]
pub struct JavascriptCommandParameters {
    script: String,
    args: Nullable<Vec<json::Json>>
}

impl Parameters for JavascriptCommandParameters {
    fn from_json(body: &json::Json) -> WebDriverResult<JavascriptCommandParameters> {
        let data = try_opt!(body.as_object(),
                            ErrorStatus::UnknownError,
                            "Message body was not an object");

        let args_json = try_opt!(data.get("args"),
                                 ErrorStatus::UnknownError,
                                 "Missing args parameter");

        let args = try!(Nullable::from_json(
            args_json,
            |x| {
                Ok((try_opt!(x.as_array(),
                             ErrorStatus::UnknownError,
                             "Failed to convert args to Array")).clone())
            }));

         //TODO: Look for WebElements in args?
        let script = try_opt!(
            try_opt!(data.get("script"),
                     ErrorStatus::UnknownError,
                     "Missing script parameter").as_string(),
            ErrorStatus::UnknownError,
            "Failed to convert script to String");
        Ok(JavascriptCommandParameters {
            script: script.to_string(),
            args: args.clone()
        })
    }
}

impl ToJson for JavascriptCommandParameters {
    fn to_json(&self) -> json::Json {
        let mut data = TreeMap::new();
        //TODO: Wrap script so that it becomes marionette-compatible
        data.insert("script".to_string(), self.script.to_json());
        data.insert("args".to_string(), self.args.to_json());
        data.insert("newSandbox".to_string(), false.to_json());
        data.insert("specialPowers".to_string(), false.to_json());
        data.insert("scriptTimeout".to_string(), json::Json::Null);
        json::Object(data)
    }
}

#[deriving(PartialEq)]
pub struct GetCookieParameters {
    name: Nullable<String>
}

impl Parameters for GetCookieParameters {
    fn from_json(body: &json::Json) -> WebDriverResult<GetCookieParameters> {
        let data = try_opt!(body.as_object(), ErrorStatus::UnknownError,
                            "Message body was not an object");
        let name_json = try_opt!(data.get("name"),
                                 ErrorStatus::InvalidArgument,
                                 "Missing 'name' parameter");
        let name = try!(Nullable::from_json(
            name_json,
            |x| {
                Ok(try_opt!(x.as_string(),
                            ErrorStatus::UnknownError,
                            "Failed to convert name to String").into_string())
            }));
        return Ok(GetCookieParameters {
            name: name
        })
    }
}

impl ToJson for GetCookieParameters {
    fn to_json(&self) -> json::Json {
        let mut data = TreeMap::new();
        data.insert("name".to_string(), self.name.to_json());
        json::Object(data)
    }
}

#[deriving(PartialEq)]
pub struct AddCookieParameters {
    pub name: String,
    pub value: String,
    pub path: Nullable<String>,
    pub domain: Nullable<String>,
    pub expiry: Nullable<Date>,
    pub maxAge: Nullable<Date>,
    pub secure: bool,
    pub httpOnly: bool
}

impl Parameters for AddCookieParameters {
    fn from_json(body: &json::Json) -> WebDriverResult<AddCookieParameters> {
        let data = try_opt!(body.as_object(),
                            ErrorStatus::UnknownError,
                            "Message body was not an object");
        let name = try_opt!(
            try_opt!(data.get("name"),
                     ErrorStatus::InvalidArgument,
                     "Missing 'name' parameter").as_string(),
            ErrorStatus::InvalidArgument,
            "'name' is not a string").into_string();

        let value = try_opt!(
            try_opt!(data.get("value"),
                     ErrorStatus::InvalidArgument,
                     "Missing 'value' parameter").as_string(),
            ErrorStatus::InvalidArgument,
            "'value' is not a string").into_string();

        let path = match data.get("path") {
            Some(path_json) => {
                try!(Nullable::from_json(
                    path_json,
                    |x| {
                        Ok(try_opt!(x.as_string(),
                                    ErrorStatus::UnknownError,
                                    "Failed to convert path to String").into_string())
                    }))
            },
            None => Nullable::Null
        };

        let domain = match data.get("domain") {
            Some(domain_json) => {
                try!(Nullable::from_json(
                    domain_json,
                    |x| {
                        Ok(try_opt!(x.as_string(),
                                    ErrorStatus::UnknownError,
                                    "Failed to convert domain to String").into_string())
                    }))
            },
            None => Nullable::Null
        };

        //TODO: This is supposed to support some text format
        let expiry = match data.get("expiry") {
            Some(expiry_json) => {
                try!(Nullable::from_json(
                    expiry_json,
                    |x| {
                        Ok(Date::new(try_opt!(x.as_u64(),
                                              ErrorStatus::UnknownError,
                                              "Failed to convert expiry to Date")))
                    }))
            },
            None => Nullable::Null
        };

        let max_age = match data.get("maxAge") {
            Some(max_age_json) => {
                try!(Nullable::from_json(
                    max_age_json,
                    |x| {
                        Ok(Date::new(try_opt!(x.as_u64(),
                                              ErrorStatus::UnknownError,
                                              "Failed to convert expiry to Date")))
                    }))
            },
            None => Nullable::Null
        };

        let secure = match data.get("secure") {
            Some(x) => try_opt!(x.as_boolean(),
                                ErrorStatus::UnknownError,
                                "Failed to convert secure to boolean"),
            None => false
        };

        let http_only = match data.get("httpOnly") {
            Some(x) => try_opt!(x.as_boolean(),
                                ErrorStatus::UnknownError,
                                "Failed to convert httpOnly to boolean"),
            None => false
        };

        return Ok(AddCookieParameters {
            name: name,
            value: value,
            path: path,
            domain: domain,
            expiry: expiry,
            maxAge: max_age,
            secure: secure,
            httpOnly: http_only
        })
    }
}

impl ToJson for AddCookieParameters {
    fn to_json(&self) -> json::Json {
        let mut data = TreeMap::new();
        data.insert("name".to_string(), self.name.to_json());
        data.insert("value".to_string(), self.value.to_json());
        data.insert("path".to_string(), self.path.to_json());
        data.insert("domain".to_string(), self.domain.to_json());
        data.insert("expiry".to_string(), self.expiry.to_json());
        data.insert("maxAge".to_string(), self.maxAge.to_json());
        data.insert("secure".to_string(), self.secure.to_json());
        data.insert("httpOnly".to_string(), self.httpOnly.to_json());
        json::Object(data)
    }
}

#[deriving(PartialEq)]
pub struct SendAlertTextParameters {
    keysToSend: String
}

impl Parameters for SendAlertTextParameters {
    fn from_json(body: &json::Json) -> WebDriverResult<SendAlertTextParameters> {
        let data = try_opt!(body.as_object(), ErrorStatus::UnknownError,
                            "Message body was not an object");
        let keys = try_opt!(
            try_opt!(data.get("keysToSend"),
                     ErrorStatus::InvalidArgument,
                     "Missing 'handle' parameter").as_string(),
            ErrorStatus::InvalidArgument,
            "'keysToSend' not a string").into_string();
        return Ok(SendAlertTextParameters {
            keysToSend: keys
        })
    }
}

impl ToJson for SendAlertTextParameters {
    fn to_json(&self) -> json::Json {
        let mut data = TreeMap::new();
        data.insert("keysToSend".to_string(), self.keysToSend.to_json());
        json::Object(data)
    }
}

#[deriving(PartialEq)]
pub struct TakeScreenshotParameters {
    pub element: Nullable<WebElement>
}

impl Parameters for TakeScreenshotParameters {
    fn from_json(body: &json::Json) -> WebDriverResult<TakeScreenshotParameters> {
        let data = try_opt!(body.as_object(), ErrorStatus::UnknownError,
                            "Message body was not an object");
        let element = match data.get("element") {
            Some(element_json) => try!(Nullable::from_json(
                element_json,
                |x| {
                    Ok(try!(WebElement::from_json(x)))
                })),
            None => Nullable::Null
        };

        return Ok(TakeScreenshotParameters {
            element: element
        })
    }
}

impl ToJson for TakeScreenshotParameters {
    fn to_json(&self) -> json::Json {
        let mut data = TreeMap::new();
        data.insert("element".to_string(), self.element.to_json());
        json::Object(data)
    }
}
