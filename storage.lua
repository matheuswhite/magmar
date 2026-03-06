local M = {}

local function get_current_dir()
    local dir = ""
    -- Use "cd" for Windows, "pwd" for Linux/macOS
    local command = (package.config:sub(1, 1) == "\\") and "cd" or "pwd"

    local pipe = io.popen(command)
    if pipe == nil then
        return dir
    end

    dir = pipe:read("*a")
    pipe:close()
    -- Trim any trailing newline characters
    return dir:gsub("\n$", ""):gsub("\r$", "")
end

local function get_filename(path)
    return path:match("^.+/(.+)$") or path
end

local function get_absolute_path(path)
    if path:sub(1, 1) == "/" then
        return path
    else
        return get_current_dir() .. "/" .. path
    end
end

local filename = ""
local oldname = ""
local fullpath = ""

local function save_file(image)
    image:encode("png", filename)
    os.rename(oldname, fullpath)
end

function M.save(path)
    fullpath = get_absolute_path(path)
    filename = get_filename(fullpath)
    local save_dir = love.filesystem.getSaveDirectory()
    oldname = save_dir .. "/" .. filename

    love.graphics.captureScreenshot(save_file)
end

return M
