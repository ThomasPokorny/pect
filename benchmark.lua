math.randomseed(os.time())

wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"

request = function()
    local key = math.random(1, 100000000)
    local value = math.random(1, 100000000)
    wrk.body = string.format('{"key": "%s", "value": "%s"}', key, value)
    return wrk.format(nil, "/pect/store")  -- '/' is the path. Modify it if necessary.
end
