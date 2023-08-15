console.log("hello world")
console.log(_VERSION)

local hello = EventNode("hello")

hello:addlistener(function(...)
  local a, b = ...
  console.log("Listener1", a + b)
end)

hello:addlistener(function(...)
  console.log("Listener2", ...)
end)

hello:addlistener(function(a, b, c)
  console.log("Listener3", a + b + c)
end)

hello:addlistener(function(a, b, c, d)
  console.log("Listener4", d)
end)

hello:invoke(1, 2, 3)
