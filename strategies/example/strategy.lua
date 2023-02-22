
-- Define inputs, get inputs (from settings file) and define default values for inputs
local rsi_periods        = input("rsi_periods", "integer", 14)
local rsi_buy_threshold  = input("rsi_buy_threshold", "integer", 30)
local rsi_sell_threshold = input("rsi_sell_threshold", "integer", 70)
local quantity           = input("order_quantity", "float", 100)

-- change environment settings
env("logging", true)                         -- will log events (orders, errors, any print calls) to file
env("data_integrity_checks", false)          -- will stop if any data integrity issues are detected
env("data_missing_mode", "smoothing")        -- will smooth out any gaps in the data as best as it can
env("history_limit", 300)                    -- set the max number of historic data to track
env("intervals", ["3m", "5m", "10m", "30m"]) -- can optionally consolidate candles to multiple timeframes (default 1m)

-- when a new bar of data is received
-- passes through the current bar and a list of up to 300 of the latest bars
function on_bar(bar, history)
    -- bar will have exchange, symbol, interval, timestamp, open, high, low, close, volume
    local exchange  = bar.exchange  -- the exchange the bar is coming from
    local symbol    = bar.symbol    -- the symbol the bar is for
    local interval  = bar.interval  -- the interval the bar is for
    local timestamp = bar.timestamp -- the timestamp of the bar
    local open      = bar.open      -- the open price of the bar
    local high      = bar.high      -- the high price of the bar
    local low       = bar.low       -- the low price of the bar
    local close     = bar.close     -- the close price of the bar
    local volume    = bar.volume    -- the volume price of the bar


    -- get the RSI indicator of the last x bars of the given timeframe of the given symbol
    -- errors if indicator is not found
    indicator("rsi", rsi_periods, interval, symbol)
    -- instead of just getting the current indicator value, we can get x number of previous values
    -- local rsi = indicator("rsi", rsi_periods, interval, symbol):history(10)

    -- if RSI is below a given threshold and we don't have a position, buy
    if rsi < rsi_buy_threshold and not get_position(exchange, symbol) then
        return command("order", (exchange, symbol, "market", quantity, "buy"))
    end

    -- if RSI is above a given threshold and we have a position, sell
    if rsi > rsi_sell_threshold and get_position(exchange, symbol) then
        return command("order", (exchange, symbol, "market", quantity, "sell"))
    end


    -- other commands we can send
    -- "insight" - log some market insight

    print("on_bar executed")
    print("bar: " .. bar)


    -- we let the system know our signal is good
    return true
end

-- when a new bar of data is received
-- passes through the order event
function on_order(order)

end