---@type table<string,function>
kissa.__loop_event__ = {}
---@meta console
console = {
  ---log
  ---@param str string
  log = function(str)
    kissa.print(str .. "\n")
  end
}
---@meta core
core = {
  ---seteventloop
  ---@param fun function
  ---@param id string
  seteventloop = function(fun, id)
    kissa.__loop_event__[id] = fun
  end,
  ---addeventloop
  ---@param fun function
  ---@return function
  addeventloop = function(fun)
    local k = kissa.newuuid()
    kissa.__loop_event__[k] = fun
    return function()
      kissa.__loop_event__[k] = nil
    end
  end
}
