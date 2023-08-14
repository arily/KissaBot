console.log("hello world")
console.log(_VERSION)

local hello = EventNode("hello")
console.debug("event:", hello)
console.log("event:", hello)

hello:addlistener(function(...)
  local a, b = ...
  console.log("Event1", a + b)
end)

hello:addlistener(function(...)
  console.log("Event2", ...)
end)

hello:invoke(1, 2, 3)
