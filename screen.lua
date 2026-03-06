local screen = {
    width = 800,
    height = 600
}
screen.__index = screen

function screen.fix_coords(x, y)
    if y == nil then
        y = x.y
        x = x.x
    end

    return {
        x = x,
        y = screen.height - y
    }
end

return screen
