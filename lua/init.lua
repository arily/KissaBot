---@meta console
console = {
  ---输出一条日志
  ---@param ... string(日志内容)
  log = function(...)
    local text = ""
    for _, value in pairs({ ... }) do
      text = text .. value .. " "
    end
    kissa.print(text .. "\n")
  end
}
---@meta core
core = {

}

--全局定义
unpack = table.unpack
