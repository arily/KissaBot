console.log("hello world")
console.log(_VERSION)

local hello = EventNode("hello")

hello:addlistener(function(args)
  local a, b = unpack(args)
  console.log("Event1", a + b)
end)

hello:addlistener(function(args)
  console.log("Event2", unpack(args))
end)

hello:invoke({ 1, 2, 3 })
