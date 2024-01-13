use core::time;
use headless_chrome::{
    browser::{
        tab::RequestPausedDecision,
        transport::{SessionId, Transport},
    },
    protocol::cdp::Fetch::{events::RequestPausedEvent, RequestPattern, RequestStage},
    types::PrintToPdfOptions,
    Browser,
};
use std::{sync::Arc, thread};

pub fn generate_pdf() -> Result<(), String> {
    let browser = Browser::default().map_err(|_| "Failed to launch browser")?;
    let tab = browser.new_tab().map_err(|_| "Failed to open tab")?;

    tab.navigate_to("file:///var/www/html/custom_dsl/desk_docviewer/ui/out/SRDocument.html")
        .map_err(|_| "Failed to navigate")?;

    let patterns = vec![RequestPattern {
        url_pattern: Some(String::from("*/characters")),
        resource_Type: None,
        request_stage: Some(RequestStage::Request),
    }];
    tab.enable_fetch(Some(&patterns), None)
        .map_err(|_| "Failed to enable fetch")?;
    tab.enable_request_interception(Arc::new(
        |_transport: Arc<Transport>, _session_id: SessionId, event: RequestPausedEvent| {
            let RequestPausedEvent { params } = event;
            println!("??????????????????? {:?}", params);
            RequestPausedDecision::Continue(None)
        },
    ))
    .map_err(|_| "Failed to enable request interception")?;

    thread::sleep(time::Duration::from_secs(2));

    let pdf = tab
        .print_to_pdf(Some(PrintToPdfOptions {
            display_header_footer: Some(false),
            margin_bottom: Some(0.0),
            margin_left: Some(0.0),
            margin_right: Some(0.0),
            margin_top: Some(0.0),
            paper_width: Some(8.3),
            paper_height: Some(11.7),
            ..Default::default()
        }))
        .map_err(|_| "Failed to print to pdf")?;

    std::fs::write("./Shrimp.pdf", pdf).map_err(|_| "Failed to write pdf")?;

    Ok(())
}
