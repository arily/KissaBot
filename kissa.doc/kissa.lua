---@meta kissa
---@type table<any,any>
kissa = {}
---@param content string
function kissa.print(content) end

---新建一个Event节点
---@param name string(name)
---@return EventNode
function EventNode(name) end

---@class EventNode
EventNode = {}
---调用该Event节点的所有函数
---@param self EventNode
---@param param table
EventNode.invoke = function(self,param) end
---增加一个函数监听该Event
---@param self EventNode
---@param func function(listener)
EventNode.addlistener = function(self, func) end
