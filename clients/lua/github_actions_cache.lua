local json = require("cjson.safe")

local M = {}

M.ENDPOINT = os.getenv("ACTIONS_CACHE_ENDPOINT") or "http://127.0.0.1:8080/actions-cache"

local function http_post(path, body_tbl)
    local body = json.encode(body_tbl or {})
    local cmd = string.format(
        "curl -sS -X POST -H 'Content-Type: application/json' -d '%s' '%s%s'",
        body, M.ENDPOINT, path
    )
    local handle = io.popen(cmd)
    local result = handle:read("*a")
    handle:close()
    return json.decode(result)
end

function M.restore(context)
    return http_post("/restore", context)
end

function M.store(context)
    return http_post("/store", context)
end

return M
