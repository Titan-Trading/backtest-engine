
-- Get settings and define default values
local interval           = get_setting("interval", "string", "1m")
local exchange           = get_setting("exchange", "string", "NYSE")
local symbol             = get_setting("symbol", "string", "EURUSD")
local rsi_periods        = get_setting("rsi_periods", "integer", 14)
local rsi_buy_threshold  = get_setting("rsi_buy_threshold", "integer", 30)
local rsi_sell_threshold = get_setting("rsi_sell_threshold", "integer", 70)
local order_quantity     = get_setting("order_quantity", "float", 0.01)

-- when a new bar of data is received
function on_bar(bar)
    -- Get the RSI indicator of the last x bars of the given timeframe of the given symbol
    local rsi = indicator("rsi", rsi_periods, interval, symbol)

    -- If RSI is below a given threshold and we don't have a position, buy
    if rsi < rsi_buy_threshold and not get_position(interval) then
        execute_market_order(interval, order_quantity, "buy")
    end

    -- If RSI is above a given threshold and we have a position, sell
    if rsi > rsi_sell_threshold and get_position(interval) then
        execute_market_order(interval, order_quantity, "sell")
    end

    print("on_bar executed")
end