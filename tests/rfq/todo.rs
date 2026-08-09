#[test]
#[ignore = "requires block-trading permissions and live RFQ counterparties"]
fn rfq_endpoints_todo() {
    // API: GET /api/v5/rfq/counterparties
    // API: POST /api/v5/rfq/create-rfq
    // API: POST /api/v5/rfq/cancel-rfq
    // API: POST /api/v5/rfq/cancel-batch-rfqs
    // API: POST /api/v5/rfq/cancel-all-rfqs
    // API: POST /api/v5/rfq/execute-quote
    // API: POST /api/v5/rfq/create-quote
    // API: POST /api/v5/rfq/cancel-quote
    // API: POST /api/v5/rfq/cancel-batch-quotes
    // API: POST /api/v5/rfq/cancel-all-quotes
    // API: GET /api/v5/rfq/rfqs
    // API: GET /api/v5/rfq/quotes
    // API: GET /api/v5/rfq/trades
    // API: GET /api/v5/rfq/public-trades
    // API: GET /api/v5/rfq/maker-instrument-settings
    // API: POST /api/v5/rfq/maker-instrument-settings
    // API: GET /api/v5/rfq/mmp-config
    // API: POST /api/v5/rfq/mmp-config
    // API: POST /api/v5/rfq/mmp-reset
    // API: POST /api/v5/rfq/cancel-all-after
    // STATUS: TODO — needs approved counterparties, maker permissions, and controlled trade caps.
    todo!("run RFQ/quote lifecycle tests in a dedicated block-trading account");
}
