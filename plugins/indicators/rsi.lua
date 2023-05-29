-- Define inputs, get inputs (from settings file) and define default values for inputs
local periods = input("periods", "integer", 14)
print(periods)

-- when a new bar of data is received
-- passes through the current bar and a list of up to 300 of the latest bars
function on_bar(bar)
    -- bar will have exchange, symbol, interval, timestamp, open, high, low, close, volume
    local exchange  = bar["exchange"]  -- the exchange the bar is coming from
    local symbol    = bar["symbol"]    -- the symbol the bar is for
    local interval  = bar["interval"]  -- the interval the bar is for
    local timestamp = bar["timestamp"] -- the timestamp of the bar
    local open      = bar["open"]      -- the open price of the bar
    local high      = bar["high"]      -- the high price of the bar
    local low       = bar["low"]       -- the low price of the bar
    local close     = bar["close"]     -- the close price of the bar
    local volume    = bar["volume"]    -- the volume price of the bar

    print("on_bar executed")
    print(dump(bar["open"]))

    -- we let the system know our signal is good
    return update({["value"] = 1})
end