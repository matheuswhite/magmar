local screen = require("screen")

local tooltip = {}
tooltip.__index = tooltip

function tooltip.new(name, color, viewport)
    local self = setmetatable({}, tooltip)

    self.width = 100
    self.height = 25
    self.name = name
    self.color = color
    self.viewport = viewport

    return self
end

function tooltip:draw(mouse, height, signal, min, max)
    if not self.viewport:is_inside(mouse) then
        return
    end

    local mouseViewport = self.viewport:screen_to_viewport(mouse)
    local mouseSignal = signal:viewport_to_signal(mouseViewport, self.viewport)

    local minStep = math.huge
    for i = 1, #signal.points - 1, 1 do
        local p1 = signal.points[i]
        local p2 = signal.points[i + 1]

        if (math.abs(p1.x - p2.x) < minStep) then
            minStep = math.abs(p1.x - p2.x)
        end
    end

    local closestPoint = nil
    for _, point in ipairs(signal.points) do
        if math.abs(point.x - mouseSignal.x) <= minStep then
            closestPoint = point
            break
        end
    end

    if closestPoint then
        local pointViewport = signal:signal_to_viewport(closestPoint, min, max, self.viewport)
        local pointScreen = self.viewport:viewport_to_screen(pointViewport)

        local point = { x = mouse.x, y = height - pointScreen.y }
        local tooltipPos = { x = point.x - self.width / 2, y = point.y - self.height - 10 }

        love.graphics.setColor(0.2, 0.2, 0.2)
        love.graphics.rectangle("fill", tooltipPos.x, tooltipPos.y, self.width, self.height)

        love.graphics.setColor(self.color)
        local text = string.format("%s: %.2f", self.name, closestPoint.y)
        love.graphics.print(text, tooltipPos.x + 5, tooltipPos.y + 5)
        love.graphics.circle("fill", point.x, point.y, 5)
    end
end

return tooltip
