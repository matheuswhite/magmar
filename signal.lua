local Tooltip = require("tooltip")

local signal = {}
signal.__index = signal

local signal_lut = {
    { 1,   1,   0 },
    { 0,   1,   1 },
    { 1,   0,   1 },
    { 1,   0.5, 0 },
    { 0.5, 1,   0 },
    { 0.5, 0,   1 },
    { 0,   0.5, 1 },
    { 1,   0,   0.5 },
    { 0,   1,   0.5 },
}

local function gen_color(index)
    return signal_lut[(index - 1) % #signal_lut + 1]
end

function signal.new(index, viewport)
    local self = setmetatable({}, signal)

    self.index = index
    self.color = gen_color(index)
    self.points = {}
    self.min = { x = 0xffffffff, y = 0xffffffff }
    self.max = { x = -0xffffffff, y = -0xffffffff }
    self.name = "Y" .. tostring(index)
    self.tooltip = Tooltip.new(self.name, self.color, viewport)

    return self
end

function signal:addPoint(x, y)
    table.insert(self.points, { x = x, y = y })
    if x < self.min.x then
        self.min.x = x
    end
    if y < self.min.y then
        self.min.y = y
    end
    if x > self.max.x then
        self.max.x = x
    end
    if y > self.max.y then
        self.max.y = y
    end
end

function signal:draw(viewport, height, min, max, mouse)
    if #self.points < 2 then
        return
    end

    love.graphics.setColor(self.color)

    for i = 1, #self.points - 1, 1 do
        local y1 = self.points[i].y
        local y2 = self.points[i + 1].y

        local x1 = self.points[i].x
        local x2 = self.points[i + 1].x

        local normalizedY1 = (y1 - min.y) / (max.y - min.y) * viewport.height + viewport.y
        local normalizedY2 = (y2 - min.y) / (max.y - min.y) * viewport.height + viewport.y

        local normalizedX1 = (x1 - min.x) / (max.x - min.x) * viewport.width + viewport.x
        local normalizedX2 = (x2 - min.x) / (max.x - min.x) * viewport.width + viewport.x

        local p1 = { x = normalizedX1, y = height - normalizedY1 }
        local p2 = { x = normalizedX2, y = height - normalizedY2 }
        love.graphics.line(p1.x, p1.y, p2.x, p2.y)
    end

    self.tooltip:draw(mouse, height, self, min, max)
end

function signal:viewport_to_signal(coords, viewport)
    local normalized = {
        x = coords.x / viewport.width,
        y = coords.y / viewport.height
    }

    return {
        x = normalized.x * (self.max.x - self.min.x) + self.min.x,
        y = normalized.y * (self.max.y - self.min.y) + self.min.y
    }
end

function signal:signal_to_viewport(coords, min, max, viewport)
    local normalized = {
        x = (coords.x - min.x) / (max.x - min.x) * viewport.width,
        y = (coords.y - min.y) / (max.y - min.y) * viewport.height
    }

    if max.x == min.x then
        normalized.x = max.x * viewport.width
    end

    if max.y == min.y then
        normalized.y = max.y * viewport.height
    end

    return normalized
end

return signal
