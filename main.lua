local Storage = require("storage")
local Signal = require("signal")
local Viewport = require("viewport")
local screen = require("screen")

local width = 800
local height = 600
local viewport = {}
local signals = {}
local read_stdin_task = nil

function love.load()
    love.graphics.setLineStyle("smooth")
    love.graphics.setLineWidth(1)

    read_stdin_task = love.thread.newThread("read_stdin.lua")
    read_stdin_task:start()

    viewport = Viewport.new(60, 25, width, height)
end

local function signals_max_y()
    local max = -0xffffffff

    for _, signal in ipairs(signals) do
        if signal.max.y > max then
            max = signal.max.y
        end
    end

    return max
end

local function signals_min_y()
    local min = 0xffffffff

    for _, signal in ipairs(signals) do
        if signal.min.y < min then
            min = signal.min.y
        end
    end

    return min
end

local function signals_max_x()
    local max = -0xffffffff

    for _, signal in ipairs(signals) do
        if signal.max.x > max then
            max = signal.max.x
        end
    end

    return max
end

local function signals_min_x()
    local min = 0xffffffff

    for _, signal in ipairs(signals) do
        if signal.min.x < min then
            min = signal.min.x
        end
    end

    return min
end

function love.draw()
    local maxY = signals_max_y()
    local minY = signals_min_y()
    local etaY = maxY * 0.01
    for i = minY, maxY + etaY, (maxY - minY) / 5 do
        local normalizedY = (i - minY) / (maxY - minY) * viewport.height + viewport.y
        love.graphics.setColor(1, 1, 1)
        love.graphics.print(string.format("%.2f", i), viewport.x - 40, height - normalizedY - 10)
        love.graphics.setColor(0.6, 0.6, 0.6)
        love.graphics.line(viewport.x, height - normalizedY, viewport.x + viewport.width, height - normalizedY)
    end

    local maxX = signals_max_x()
    local minX = signals_min_x()
    local etaX = maxX * 0.01
    for i = minX, maxX + etaX, (maxX - minX) / 5 do
        local normalizedX = (i - minX) / (maxX - minX) * viewport.width + viewport.x
        love.graphics.setColor(1, 1, 1)
        love.graphics.print(string.format("%.2f", i), normalizedX - 10, height - viewport.y + 10)
        love.graphics.setColor(0.6, 0.6, 0.6)
        love.graphics.line(normalizedX, height - viewport.y, normalizedX, height - viewport.y - viewport.height)
    end

    love.graphics.setColor(1, 1, 1)
    local scale = 1.3
    love.graphics.print("Time", width / 2, height - 35, 0, scale, scale)
    love.graphics.print("Signal", 15, height / 2, -math.pi / 2, scale, scale)

    local min = { x = minX, y = minY }
    local max = { x = maxX, y = maxY }

    local x, y = love.mouse.getPosition()
    local mouse = screen.fix_coords(x, y)

    for _, signal in ipairs(signals) do
        signal:draw(viewport, height, min, max, mouse)
    end
end

function love.update(dt)
    local pointsChannel = love.thread.getChannel('points')
    while pointsChannel:getCount() > 0 do
        local pointData = pointsChannel:pop()
        if #signals < #pointData.signals then
            for i = #signals + 1, #pointData.signals do
                table.insert(signals, Signal.new(i, viewport))
            end
        end

        for i, signal in ipairs(signals) do
            local signalValue = pointData.signals[i] or 0
            signal:addPoint(pointData.time, signalValue)
        end
    end

    local cmdChannel = love.thread.getChannel('cmd')
    while cmdChannel:getCount() > 0 do
        local command = cmdChannel:pop()
        if command.action == "save" then
            local filename = command.filename
            Storage.save(filename)
            print("Saved screenshot as " .. filename)
        end
    end
end
