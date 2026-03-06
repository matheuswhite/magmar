local viewport = {}
viewport.__index = viewport

function viewport.new(padding, offset, screen_width, screen_height)
    local self = setmetatable({}, viewport)

    self.x = padding + offset
    self.y = padding + offset
    self.width = screen_width - 2 * padding
    self.height = screen_height - 2 * padding

    return self
end

function viewport:screen_to_viewport(coords)
    local viewportCoords = {
        x = coords.x - self.x,
        y = coords.y - self.y
    }

    return viewportCoords
end

function viewport:viewport_to_screen(coords)
    local screenCoords = {
        x = coords.x + self.x,
        y = coords.y + self.y
    }

    return screenCoords
end

function viewport:is_inside(coords)
    return coords.x >= self.x and coords.x <= self.x + self.width and
        coords.y >= self.y and coords.y <= self.y + self.height
end

return viewport
