local pointsChannel = love.thread.getChannel('points')
local cmdChannel = love.thread.getChannel('cmd')

while true do
    local line = io.read()
    if line then
        local splitted = {}
        for value in string.gmatch(line, "([^,]+)") do
            table.insert(splitted, value)
        end

        if splitted[1] == "!save" then
            local filename = splitted[2] or "points.png"
            cmdChannel:push({ action = "save", filename = filename })
        else
            local time = tonumber(splitted[1])
            if time ~= nil then
                local signals = {}
                for i = 2, #splitted do
                    local signalValue = tonumber(splitted[i])
                    table.insert(signals, signalValue or 0)
                end

                pointsChannel:push({ time = time, signals = signals })
            end
        end
    end
end
